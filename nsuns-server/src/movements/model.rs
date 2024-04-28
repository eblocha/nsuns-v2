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
    assert_all_owner, assert_owner,
    auth::token::OwnerId,
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
        owner_id: OwnerId,
        executor: impl Executor<'_, Database = DB>,
    ) -> OperationResult<Vec<Self>> {
        sqlx::query_as::<_, Self>(formatcp!("{SELECT} * FROM {TABLE} WHERE owner_id = $1"))
            .bind(owner_id)
            .fetch_all(executor.instrument_executor(db_span!(SELECT, TABLE)))
            .await
            .context("failed to select movements")
            .map_err(into_log_server_error!())
    }

    pub async fn update_one(
        self,
        owner_id: OwnerId,
        executor: impl Executor<'_, Database = DB>,
    ) -> OperationResult<Option<Self>> {
        sqlx::query(formatcp!(
            "{UPDATE} {TABLE} SET name = $1, description = $2 WHERE id = $3 AND owner_id = $4"
        ))
        .bind(&self.name)
        .bind(self.description.as_ref())
        .bind(self.id)
        .bind(owner_id)
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

    pub async fn assert_owner(
        id: Uuid,
        owner_id: OwnerId,
        executor: impl Executor<'_, Database = DB>,
    ) -> OperationResult<()> {
        assert_owner!(TABLE, "movement", id, owner_id, executor)
    }

    pub async fn assert_all_owner(
        ids: &[Uuid],
        owner_id: OwnerId,
        executor: impl Executor<'_, Database = DB>,
    ) -> OperationResult<()> {
        assert_all_owner!(TABLE, "movement", ids, owner_id, executor)
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
        owner_id: OwnerId,
        executor: impl Executor<'_, Database = DB>,
    ) -> OperationResult<Movement> {
        sqlx::query_as::<_, Movement>(formatcp!(
            "{INSERT_INTO} {TABLE} (name, description, owner_id) VALUES ($1, $2, $3) RETURNING *",
        ))
        .bind(&self.name)
        .bind(self.description.as_ref())
        .bind(owner_id)
        .fetch_one(executor.instrument_executor(db_span!(INSERT_INTO, TABLE)))
        .await
        .map_err(|e| handle_error(e, || "failed to insert new movement"))
        .map_err(log_server_error!())
    }

    pub async fn insert_many(
        movements: &[Self],
        owner_id: OwnerId,
        executor: impl Executor<'_, Database = DB>,
    ) -> OperationResult<Vec<Movement>> {
        // sqlx does not yet support providing iterators for bound data.
        let names: Vec<_> = movements.iter().map(|m| m.name.as_str()).collect();
        let descriptions: Vec<_> = movements.iter().map(|m| m.description.as_deref()).collect();

        sqlx::query_as::<_, Movement>(formatcp!(
            "{INSERT_INTO} {TABLE} (name, description, owner_id)
            VALUES (
                unnest($1),
                unnest($2),
                $3
            )
            RETURNING *"
        ))
        .bind(&names)
        .bind(&descriptions)
        .bind(owner_id)
        .fetch_all(executor.instrument_executor(db_span!(INSERT_INTO, TABLE)))
        .await
        .map_err(|e| handle_error(e, || "failed to insert new movements"))
        .map_err(log_server_error!())
    }
}
