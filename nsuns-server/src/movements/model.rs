use anyhow::{anyhow, Context};
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::Executor;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::{
    db::DB,
    error::{ext_context, ErrorWithStatus, OperationResult},
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

fn not_unique_error() -> ErrorWithStatus<anyhow::Error> {
    ErrorWithStatus {
        status: StatusCode::CONFLICT,
        error: anyhow!("movement name is not unique"),
    }
}

impl Movement {
    pub async fn select_all(
        executor: impl Executor<'_, Database = DB>,
    ) -> OperationResult<Vec<Self>> {
        sqlx::query_as::<_, Self>("SELECT * FROM movements")
            .fetch_all(executor)
            .await
            .with_context(|| "failed to select movements")
            .map_err(Into::into)
    }

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
            .map_err(|e| match e {
                sqlx::Error::Database(e) if e.is_unique_violation() => not_unique_error(),
                _ => ext_context(
                    e,
                    format!("failed to update movement with id={id}", id = self.id),
                )
                .into(),
            })
            .map(|result| {
                if result.rows_affected() == 0 {
                    None
                } else {
                    Some(self)
                }
            })
            .map_err(Into::into)
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
        sqlx::query_as::<_, Movement>(
            "INSERT INTO movements (name, description) VALUES ($1, $2) RETURNING *",
        )
        .bind(&self.name)
        .bind(self.description.as_ref())
        .fetch_one(executor)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(e) if e.is_unique_violation() => not_unique_error(),
            _ => ext_context(e, "failed to insert new movement").into(),
        })
        .map_err(Into::into)
    }
}
