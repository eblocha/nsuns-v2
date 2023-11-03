use std::fmt::Display;

use anyhow::{anyhow, Context};
use axum::http::StatusCode;
use chrono::naive::serde::ts_milliseconds;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use sqlx::Executor;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::{
    db::DB,
    error::{ErrorWithStatus, OperationResult},
};

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow, ToSchema, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Max {
    #[schema(value_type = String, format = Int64)]
    #[serde_as(as = "DisplayFromStr")]
    pub id: i64,
    pub profile_id: Uuid,
    pub movement_id: Uuid,
    pub amount: f64,
    #[schema(value_type = i64)]
    #[serde(with = "ts_milliseconds")]
    pub timestamp: NaiveDateTime,
}

fn handle_error<F, C>(e: sqlx::Error, context: F) -> ErrorWithStatus<anyhow::Error>
where
    F: FnOnce() -> C,
    C: Display + Send + Sync + 'static,
{
    match e {
        sqlx::Error::Database(e) if e.is_foreign_key_violation() => ErrorWithStatus {
            status: StatusCode::BAD_REQUEST,
            error: anyhow!("movementId or profileId provided does not exist"),
        },
        _ => anyhow!(e).context(context()).into(),
    }
}

impl Max {
    #[tracing::instrument(name = "Max::select_for_profile", skip_all)]
    pub async fn select_for_profile(
        profile_id: Uuid,
        executor: impl Executor<'_, Database = DB>,
    ) -> OperationResult<Vec<Self>> {
        sqlx::query_as::<_, Self>("SELECT * FROM maxes WHERE profile_id = $1 ORDER BY timestamp")
            .bind(profile_id)
            .fetch_all(executor)
            .await
            .with_context(|| format!("failed to select maxes for profile_id={profile_id}"))
            .map_err(Into::into)
    }

    #[tracing::instrument(name = "Max::select_latest", skip_all)]
    pub async fn select_latest(
        movement_id: Uuid,
        profile_id: Uuid,
        executor: impl Executor<'_, Database = DB>,
    ) -> OperationResult<Option<Self>> {
        sqlx::query_as::<_, Self>("SELECT * FROM maxes WHERE movement_id = $1 AND profile_id = $2 ORDER BY timestamp DESC LIMIT 1")
            .bind(movement_id)
            .bind(profile_id)
            .fetch_optional(executor)
            .await
            .with_context(|| format!("failed to fetch latest max for profile_id={profile_id} and movement_id={movement_id}"))
            .map_err(Into::into)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateMax {
    pub profile_id: Uuid,
    pub movement_id: Uuid,
    #[validate(range(min = 0))]
    pub amount: f64,
}

impl CreateMax {
    #[tracing::instrument(name = "CreateMax::insert_one", skip_all)]
    pub async fn insert_one(
        self,
        executor: impl Executor<'_, Database = DB>,
    ) -> OperationResult<Max> {
        sqlx::query_as::<_, (i64, NaiveDateTime)>(
            "INSERT INTO maxes (profile_id, movement_id, amount) VALUES ($1, $2, $3) RETURNING id, timestamp",
        )
        .bind(self.profile_id)
        .bind(self.movement_id)
        .bind(self.amount)
        .fetch_one(executor)
        .await
        .map_err(|e| {
            handle_error(e, || "failed to insert a new max")
        })
        .map(|(id, timestamp)| Max {
            id,
            profile_id: self.profile_id,
            movement_id: self.movement_id,
            amount: self.amount,
            timestamp
        })
    }
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateMax {
    #[schema(value_type = String, format = Int64)]
    #[serde_as(as = "DisplayFromStr")]
    pub id: i64,
    #[validate(range(min = 0))]
    pub amount: f64,
}

impl UpdateMax {
    #[tracing::instrument(name = "UpdateMax::update_one", skip_all)]
    pub async fn update_one(
        self,
        executor: impl Executor<'_, Database = DB>,
    ) -> OperationResult<Option<Max>> {
        sqlx::query_as::<_, Max>("UPDATE maxes SET amount = $1 WHERE id = $2 RETURNING *")
            .bind(self.amount)
            .bind(self.id)
            .fetch_optional(executor)
            .await
            .map_err(|e| {
                handle_error(e, || {
                    format!("failed to update max with id={id}", id = self.id)
                })
            })
    }
}

#[tracing::instrument(skip_all)]
pub async fn delete_latest_maxes(
    profile_id: Uuid,
    movement_id: Uuid,
    executor: impl Executor<'_, Database = DB>,
) -> OperationResult<Option<i64>> {
    sqlx::query_as::<_, (i64,)>(
        "DELETE FROM maxes WHERE id = any(
        array(SELECT id FROM maxes WHERE movement_id = $1 AND profile_id = $2 ORDER BY timestamp DESC LIMIT 1)
    ) RETURNING id",)
    .bind(movement_id)
    .bind(profile_id)
    .fetch_optional(executor)
    .await
    .map(|res| res.map(|(id,)| id))
    .with_context(|| "failed to delete latest maxes")
    .map_err(Into::into)
}
