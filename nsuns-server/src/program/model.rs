use std::fmt::Display;

use anyhow::{anyhow, Context};
use axum::http::StatusCode;
use chrono::naive::serde::ts_milliseconds;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{Executor, Transaction};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::{
    db::DB,
    error::{ErrorWithStatus, OperationResult},
    sets::model::Set,
};

fn handle_error<F, C>(e: sqlx::Error, context: F) -> ErrorWithStatus<anyhow::Error>
where
    F: FnOnce() -> C,
    C: Display + Send + Sync + 'static,
{
    match e {
        sqlx::Error::Database(e) if e.is_foreign_key_violation() => ErrorWithStatus {
            status: StatusCode::BAD_REQUEST,
            error: anyhow!("profileId provided does not exist"),
        },
        _ => anyhow!(e).context(context()).into(),
    }
}

#[derive(Debug, Clone, sqlx::FromRow)]
struct Program {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub owner: Uuid,
    pub created_on: NaiveDateTime,
    pub set_ids_sunday: Vec<Uuid>,
    pub set_ids_monday: Vec<Uuid>,
    pub set_ids_tuesday: Vec<Uuid>,
    pub set_ids_wednesday: Vec<Uuid>,
    pub set_ids_thursday: Vec<Uuid>,
    pub set_ids_friday: Vec<Uuid>,
    pub set_ids_saturday: Vec<Uuid>,
}

impl Program {
    pub async fn select_one(
        id: Uuid,
        executor: impl Executor<'_, Database = DB>,
    ) -> OperationResult<Option<Self>> {
        sqlx::query_as::<_, Self>("SELECT * from programs WHERE id = $1")
            .bind(id)
            .fetch_optional(executor)
            .await
            .with_context(|| format!("failed to fetch program with id={id}"))
            .map_err(Into::into)
    }

    pub fn all_set_ids(&self) -> Vec<Uuid> {
        [
            self.set_ids_sunday.to_owned(),
            self.set_ids_monday.to_owned(),
            self.set_ids_tuesday.to_owned(),
            self.set_ids_wednesday.to_owned(),
            self.set_ids_thursday.to_owned(),
            self.set_ids_friday.to_owned(),
            self.set_ids_saturday.to_owned(),
        ]
        .concat()
    }
}

#[derive(Debug, Serialize, Clone, sqlx::FromRow, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ProgramMeta {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub owner: Uuid,
    #[schema(value_type = i64)]
    #[serde(with = "ts_milliseconds")]
    pub created_on: NaiveDateTime,
}

impl ProgramMeta {
    pub async fn select_all_for_profile(
        executor: impl Executor<'_, Database = DB>,
        owner: &Uuid,
    ) -> OperationResult<Vec<Self>> {
        sqlx::query_as::<_, Self>("SELECT * FROM programs WHERE owner = $1 ORDER BY created_on")
            .bind(owner)
            .fetch_all(executor)
            .await
            .with_context(|| format!("failed to select program with owner id={owner}"))
            .map_err(Into::into)
    }

    pub async fn select_one(
        id: Uuid,
        executor: impl Executor<'_, Database = DB>,
    ) -> OperationResult<Option<Self>> {
        sqlx::query_as::<_, Self>("SELECT * from programs WHERE id = $1")
            .bind(id)
            .fetch_optional(executor)
            .await
            .with_context(|| format!("failed to fetch program with id={id}"))
            .map_err(Into::into)
    }
}

impl From<Program> for ProgramMeta {
    fn from(value: Program) -> Self {
        ProgramMeta {
            id: value.id,
            name: value.name,
            description: value.description,
            owner: value.owner,
            created_on: value.created_on,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateProgram {
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(length(min = 1))]
    pub description: Option<String>,
    pub owner: Uuid,
}

impl CreateProgram {
    pub async fn insert_one(
        self,
        executor: impl Executor<'_, Database = DB>,
    ) -> OperationResult<ProgramMeta> {
        sqlx::query_as::<_, ProgramMeta>(
            "INSERT INTO programs (name, description, owner) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(self.name)
        .bind(self.description)
        .bind(self.owner)
        .fetch_one(executor)
        .await
        .map_err(|e| handle_error(e, || "failed to create program"))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProgram {
    pub id: Uuid,
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(length(min = 1))]
    pub description: Option<String>,
}

impl UpdateProgram {
    pub async fn update_one(
        self,
        executor: impl Executor<'_, Database = DB>,
    ) -> OperationResult<Option<ProgramMeta>> {
        sqlx::query_as::<_, ProgramMeta>(
            "UPDATE programs SET name = $1, description = $2 WHERE id = $3 RETURNING *",
        )
        .bind(self.name)
        .bind(self.description)
        .bind(self.id)
        .fetch_optional(executor)
        .await
        .map_err(|e| {
            handle_error(e, || {
                format!("failed to update program with id={id}", id = self.id)
            })
        })
    }
}

pub async fn delete_one(
    id: Uuid,
    tx: &mut Transaction<'_, DB>,
) -> OperationResult<Option<ProgramMeta>> {
    let program = sqlx::query_as::<_, Program>("DELETE FROM programs WHERE id = $1 RETURNING *")
        .bind(id)
        .fetch_optional(&mut **tx)
        .await
        .with_context(|| format!("failed to delete program with id={id}"))?;

    if let Some(program) = program {
        let set_ids = program.all_set_ids();

        Set::delete_where_id_in(&set_ids, &mut **tx)
            .await
            .with_context(|| format!("failed to delete sets under program with id={id}"))?;
        Ok(Some(program.into()))
    } else {
        Ok(None)
    }
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ProgramSummary {
    pub program: ProgramMeta,
    pub sets_sunday: Vec<Set>,
    pub sets_monday: Vec<Set>,
    pub sets_tuesday: Vec<Set>,
    pub sets_wednesday: Vec<Set>,
    pub sets_thursday: Vec<Set>,
    pub sets_friday: Vec<Set>,
    pub sets_saturday: Vec<Set>,
}

pub async fn gather_program_summary(
    id: Uuid,
    tx: &mut Transaction<'_, DB>,
) -> OperationResult<Option<ProgramSummary>> {
    let program_opt = Program::select_one(id, &mut **tx).await?;

    let get_ctx = || format!("failed to fetch sets for program with id={id}");

    if let Some(program) = program_opt {
        let sets_sunday = Set::select_where_id_in(&program.set_ids_sunday, &mut **tx)
            .await
            .with_context(get_ctx)?;

        let sets_monday = Set::select_where_id_in(&program.set_ids_monday, &mut **tx)
            .await
            .with_context(get_ctx)?;

        let sets_tuesday = Set::select_where_id_in(&program.set_ids_tuesday, &mut **tx)
            .await
            .with_context(get_ctx)?;

        let sets_wednesday = Set::select_where_id_in(&program.set_ids_wednesday, &mut **tx)
            .await
            .with_context(get_ctx)?;

        let sets_thursday = Set::select_where_id_in(&program.set_ids_thursday, &mut **tx)
            .await
            .with_context(get_ctx)?;

        let sets_friday = Set::select_where_id_in(&program.set_ids_friday, &mut **tx)
            .await
            .with_context(get_ctx)?;

        let sets_saturday = Set::select_where_id_in(&program.set_ids_saturday, &mut **tx)
            .await
            .with_context(get_ctx)?;

        Ok(Some(ProgramSummary {
            program: program.into(),
            sets_sunday,
            sets_monday,
            sets_tuesday,
            sets_wednesday,
            sets_thursday,
            sets_friday,
            sets_saturday,
        }))
    } else {
        Ok(None)
    }
}
