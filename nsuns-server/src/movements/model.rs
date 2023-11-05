use std::fmt::Display;

use anyhow::{anyhow, Context};
use axum::http::StatusCode;
use const_format::formatcp;
use serde::{Deserialize, Serialize};
use sqlx::Executor;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::{
    db::{
        tracing::{
            statements::{INSERT_INTO, SELECT, UPDATE},
            InstrumentExecutor,
        },
        DB,
    },
    db_span,
    error::{ErrorWithStatus, OperationResult},
    into_log_server_error, log_server_error,
};

const TABLE: &str = "movements";

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Movement {
    pub id: Uuid,
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(length(min = 1))]
    pub description: Option<String>,
}

fn handle_error<F, C>(e: sqlx::Error, context: F) -> ErrorWithStatus<anyhow::Error>
where
    F: FnOnce() -> C,
    C: Display + Send + Sync + 'static,
{
    match e {
        sqlx::Error::Database(e) if e.is_unique_violation() => {
            ErrorWithStatus::new(StatusCode::CONFLICT, anyhow!("movement name is not unique"))
        }
        _ => anyhow!(e).context(context()).into(),
    }
}

impl Movement {
    pub async fn select_all(
        executor: impl Executor<'_, Database = DB>,
    ) -> OperationResult<Vec<Self>> {
        sqlx::query_as::<_, Self>(formatcp!("{SELECT} * FROM {TABLE}"))
            .fetch_all(executor.instrument_executor(db_span!(SELECT, TABLE)))
            .await
            .with_context(|| "failed to select movements")
            .map_err(into_log_server_error!())
    }

    pub async fn update_one(
        self,
        executor: impl Executor<'_, Database = DB>,
    ) -> OperationResult<Option<Self>> {
        sqlx::query(formatcp!(
            "{UPDATE} {TABLE} SET name = $1, description = $2 WHERE id = $3"
        ))
        .bind(&self.name)
        .bind(self.description.as_ref())
        .bind(self.id)
        .execute(executor.instrument_executor(db_span!(UPDATE, TABLE)))
        .await
        .map_err(|e| {
            handle_error(e, || {
                format!("failed to update movement with id={id}", id = self.id)
            })
        })
        .map(|result| {
            if result.rows_affected() == 0 {
                None
            } else {
                Some(self)
            }
        })
        .map_err(log_server_error!())
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateMovement {
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(length(min = 1))]
    pub description: Option<String>,
}

impl CreateMovement {
    pub async fn insert_one(
        self,
        executor: impl Executor<'_, Database = DB>,
    ) -> OperationResult<Movement> {
        sqlx::query_as::<_, Movement>(formatcp!(
            "{INSERT_INTO} {TABLE} (name, description) VALUES ($1, $2) RETURNING *",
        ))
        .bind(&self.name)
        .bind(self.description.as_ref())
        .fetch_one(executor.instrument_executor(db_span!(INSERT_INTO, TABLE)))
        .await
        .map_err(|e| handle_error(e, || "failed to insert new movement"))
        .map_err(log_server_error!())
    }
}
