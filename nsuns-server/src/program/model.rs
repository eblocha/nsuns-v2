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

#[derive(Debug, Serialize, Clone, sqlx::FromRow, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Program {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub owner: Uuid,
    #[schema(value_type = i64)]
    #[serde(with = "ts_milliseconds")]
    pub created_on: NaiveDateTime,
}

impl Program {
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
    ) -> OperationResult<Program> {
        sqlx::query_as::<_, Program>(
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
    ) -> OperationResult<Option<Program>> {
        sqlx::query_as::<_, Program>(
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
    executor: impl Executor<'_, Database = DB>,
) -> OperationResult<Option<Program>> {
    sqlx::query_as::<_, Program>("DELETE FROM programs WHERE id = $1 RETURNING *")
        .bind(id)
        .fetch_optional(executor)
        .await
        .with_context(|| format!("failed to delete program with id={id}"))
        .map_err(Into::into)
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ProgramSummary {
    pub program: Program,
    pub sets: Vec<Set>,
}

pub async fn gather_program_summary(
    id: Uuid,
    tx: &mut Transaction<'_, DB>,
) -> OperationResult<Option<ProgramSummary>> {
    let program_opt = Program::select_one(id, &mut **tx).await?;

    if let Some(program) = program_opt {
        let sets = Set::select_for_program(program.id, &mut **tx).await?;
        Ok(Some(ProgramSummary { program, sets }))
    } else {
        Ok(None)
    }
}
