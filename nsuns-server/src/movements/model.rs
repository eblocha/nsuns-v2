use std::fmt::Display;

use anyhow::{anyhow, Context};
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::Executor;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::{
    db::DB,
    error::{ErrorWithStatus, OperationResult},
};

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
        sqlx::Error::Database(e) if e.is_unique_violation() => ErrorWithStatus {
            status: StatusCode::CONFLICT,
            error: anyhow!("movement name is not unique"),
        },
        _ => anyhow!(e).context(context()).into(),
    }
}

impl Movement {
    #[tracing::instrument(name = "Movement::select_all", skip(executor))]
    pub async fn select_all(
        executor: impl Executor<'_, Database = DB>,
    ) -> OperationResult<Vec<Self>> {
        sqlx::query_as::<_, Self>("SELECT * FROM movements")
            .fetch_all(executor)
            .await
            .with_context(|| "failed to select movements")
            .map_err(Into::into)
    }

    #[tracing::instrument(name = "Movement::update_one", skip(self, executor), fields(movement_id = %self.id))]
    pub async fn update_one(
        self,
        executor: impl Executor<'_, Database = DB>,
    ) -> OperationResult<Option<Self>> {
        sqlx::query("UPDATE movements SET name = $1, description = $2 WHERE id = $3")
            .bind(&self.name)
            .bind(self.description.as_ref())
            .bind(self.id)
            .execute(executor)
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
    #[tracing::instrument(name = "CreateMovement::insert_one", skip(self, executor))]
    pub async fn insert_one(
        self,
        executor: impl Executor<'_, Database = DB>,
    ) -> OperationResult<Movement> {
        sqlx::query_as::<_, Movement>(
            "INSERT INTO movements (name, description) VALUES ($1, $2) RETURNING *",
        )
        .bind(&self.name)
        .bind(self.description.as_ref())
        .fetch_one(executor)
        .await
        .map_err(|e| handle_error(e, || "failed to insert new movement"))
    }
}
