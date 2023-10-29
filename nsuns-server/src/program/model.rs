use std::fmt::Display;

use anyhow::{anyhow, Context};
use axum::http::StatusCode;
use chrono::naive::serde::ts_milliseconds;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgQueryResult, Executor, Transaction};
use tracing::Instrument;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::{
    db::DB,
    error::{ErrorWithStatus, OperationResult},
    sets::model::{Day, Set},
    vec::MoveWithin,
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
    #[tracing::instrument(name = "Program::select_one", skip(executor))]
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

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow, ToSchema, PartialEq)]
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
    #[tracing::instrument(name = "ProgramMeta::select_all_for_profile", skip(executor))]
    pub async fn select_all_for_profile(
        executor: impl Executor<'_, Database = DB>,
        owner: &Uuid,
    ) -> OperationResult<Vec<Self>> {
        sqlx::query_as::<_, Self>(
            "SELECT
            id,
            name,
            description,
            owner,
            created_on
            FROM programs WHERE owner = $1 ORDER BY created_on",
        )
        .bind(owner)
        .fetch_all(executor)
        .await
        .with_context(|| format!("failed to select program with owner id={owner}"))
        .map_err(Into::into)
    }

    #[tracing::instrument(name = "ProgramMeta::select_one", skip(executor))]
    pub async fn select_one(
        id: Uuid,
        executor: impl Executor<'_, Database = DB>,
    ) -> OperationResult<Option<Self>> {
        sqlx::query_as::<_, Self>(
            "SELECT
            id,
            name,
            description,
            owner,
            created_on
            FROM programs WHERE id = $1",
        )
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
    #[tracing::instrument(name = "CreateProgram::insert_one", skip(self, executor))]
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
    #[tracing::instrument(name = "UpdateProgram::update_one", skip(self, executor), fields(id = %self.id))]
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

#[tracing::instrument(name = "Program::delete_one", skip(executor))]
pub async fn delete_one(
    id: Uuid,
    executor: impl Executor<'_, Database = DB>,
) -> OperationResult<Option<ProgramMeta>> {
    sqlx::query_as::<_, ProgramMeta>("DELETE FROM programs WHERE id = $1 RETURNING *")
        .bind(id)
        .fetch_optional(executor)
        .await
        .with_context(|| format!("failed to delete program with id={id}"))
        .map_err(Into::into)
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
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

#[tracing::instrument(skip(tx))]
pub async fn gather_program_summary(
    id: Uuid,
    tx: &mut Transaction<'_, DB>,
) -> OperationResult<Option<ProgramSummary>> {
    let program_opt = Program::select_one(id, &mut **tx).await?;

    let get_ctx = || format!("failed to fetch sets for program with id={id}");

    if let Some(program) = program_opt {
        let sets_sunday = Set::select_where_id_in(&program.set_ids_sunday, &mut **tx)
            .instrument(tracing::info_span!("select sets for sunday"))
            .await
            .with_context(get_ctx)?;

        let sets_monday = Set::select_where_id_in(&program.set_ids_monday, &mut **tx)
            .instrument(tracing::info_span!("select sets for monday"))
            .await
            .with_context(get_ctx)?;

        let sets_tuesday = Set::select_where_id_in(&program.set_ids_tuesday, &mut **tx)
            .instrument(tracing::info_span!("select sets for tuesday"))
            .await
            .with_context(get_ctx)?;

        let sets_wednesday = Set::select_where_id_in(&program.set_ids_wednesday, &mut **tx)
            .instrument(tracing::info_span!("select sets for wednesday"))
            .await
            .with_context(get_ctx)?;

        let sets_thursday = Set::select_where_id_in(&program.set_ids_thursday, &mut **tx)
            .instrument(tracing::info_span!("select sets for thursday"))
            .await
            .with_context(get_ctx)?;

        let sets_friday = Set::select_where_id_in(&program.set_ids_friday, &mut **tx)
            .instrument(tracing::info_span!("select sets for friday"))
            .await
            .with_context(get_ctx)?;

        let sets_saturday = Set::select_where_id_in(&program.set_ids_saturday, &mut **tx)
            .instrument(tracing::info_span!("select sets for saturday"))
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

fn get_day_column(day: Day) -> &'static str {
    match day {
        Day::Sunday => "set_ids_sunday",
        Day::Monday => "set_ids_monday",
        Day::Tuesday => "set_ids_tuesday",
        Day::Wednesday => "set_ids_wednesday",
        Day::Thursday => "set_ids_thursday",
        Day::Friday => "set_ids_friday",
        Day::Saturday => "set_ids_saturday",
    }
}

#[tracing::instrument(skip(executor))]
pub async fn get_set_ids(
    program_id: Uuid,
    day: Day,
    for_update: bool,
    executor: impl Executor<'_, Database = DB>,
) -> OperationResult<Option<Vec<Uuid>>> {
    let day_col = get_day_column(day);
    let lock_clause = if for_update { "FOR UPDATE" } else { "" };

    let set_ids = sqlx::query_as::<_, (Vec<Uuid>,)>(&format!(
        "SELECT {day_col} FROM programs WHERE id = $1 {lock_clause}",
    ))
    .bind(program_id)
    .fetch_optional(executor)
    .await
    .with_context(|| {
        format!("failed to fetch existing set ids for day={day:?} and program_id={program_id}",)
    })?
    .map(|id| id.0);

    Ok(set_ids)
}

#[tracing::instrument(skip(executor))]
pub async fn update_set_ids(
    program_id: Uuid,
    day: Day,
    set_ids: &Vec<Uuid>,
    executor: impl Executor<'_, Database = DB>,
) -> OperationResult<PgQueryResult> {
    let day_col = get_day_column(day);
    sqlx::query(&format!("UPDATE programs SET {day_col} = $1 WHERE id = $2"))
        .bind(set_ids)
        .bind(program_id)
        .execute(executor)
        .await
        .with_context(|| {
            format!("failed to update set ids for day={day:?} and program_id={program_id}",)
        })
        .map_err(Into::into)
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ReorderSets {
    pub program_id: Uuid,
    #[schema(value_type = i16)]
    pub day: Day,
    #[validate(range(min = 0))]
    pub from: usize,
    #[validate(range(min = 0))]
    pub to: usize,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SetId(Uuid);

impl ReorderSets {

    #[tracing::instrument(name = "ReorderSets::reorder", skip(self, tx))]
    pub async fn reorder<'a>(
        &self,
        tx: &mut Transaction<'a, DB>,
    ) -> OperationResult<Option<Vec<SetId>>> {
        if let Some(mut set_ids) = get_set_ids(self.program_id, self.day, true, &mut **tx).await? {
            if self.from >= set_ids.len() || self.to >= set_ids.len() {
                return Err(ErrorWithStatus {
                    status: StatusCode::CONFLICT,
                    error: anyhow!("index out of bounds"),
                });
            }

            if set_ids.move_within(self.from, self.to) {
                update_set_ids(self.program_id, self.day, &set_ids, &mut **tx).await?;
            }

            return Ok(Some(set_ids.into_iter().map(SetId).collect()));
        }

        Ok(None)
    }
}
