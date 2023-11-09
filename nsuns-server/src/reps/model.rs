use std::fmt::Display;

use anyhow::{anyhow, Context};
use axum::http::StatusCode;
use chrono::naive::serde::ts_milliseconds;
use chrono::NaiveDateTime;
use const_format::formatcp;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use sqlx::Executor;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::{
    db::{
        tracing::{
            statements::{DELETE_FROM, INSERT_INTO, SELECT, UPDATE},
            InstrumentExecutor,
        },
        DB,
    },
    db_span,
    error::{ErrorWithStatus, OperationResult},
    into_log_server_error, log_server_error,
};

const TABLE: &str = "reps";

fn handle_error<F, C>(e: sqlx::Error, context: F) -> ErrorWithStatus<anyhow::Error>
where
    F: FnOnce() -> C,
    C: Display + Send + Sync + 'static,
{
    match e {
        sqlx::Error::Database(e) if e.is_foreign_key_violation() => ErrorWithStatus::new(
            StatusCode::BAD_REQUEST,
            anyhow!("movementId or profileId provided does not exist"),
        ),
        _ => anyhow!(e).context(context()).into(),
    }
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow, ToSchema, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Reps {
    #[schema(value_type = String, format = Int64)]
    #[serde_as(as = "DisplayFromStr")]
    pub id: i64,
    pub profile_id: Uuid,
    pub movement_id: Uuid,
    pub amount: Option<i32>,
    #[schema(value_type = i64)]
    #[serde(with = "ts_milliseconds")]
    pub timestamp: NaiveDateTime,
}

impl Reps {
    pub async fn select_for_profile(
        profile_id: Uuid,
        executor: impl Executor<'_, Database = DB>,
    ) -> OperationResult<Vec<Self>> {
        sqlx::query_as::<_, Self>(formatcp!(
            "{SELECT} * FROM {TABLE} WHERE profile_id = $1 ORDER BY timestamp"
        ))
        .bind(profile_id)
        .fetch_all(executor.instrument_executor(db_span!(SELECT, TABLE)))
        .await
        .with_context(|| format!("failed to select reps for profile_id={profile_id}"))
        .map_err(into_log_server_error!())
    }

    pub async fn select_latest(
        movement_id: Uuid,
        profile_id: Uuid,
        executor: impl Executor<'_, Database = DB>,
    ) -> OperationResult<Option<Self>> {
        sqlx::query_as::<_, Self>(formatcp!(
                "{SELECT} * FROM {TABLE} WHERE movement_id = $1 AND profile_id = $2 ORDER BY timestamp DESC LIMIT 1"
            ))
            .bind(movement_id)
            .bind(profile_id)
            .fetch_optional(executor.instrument_executor(db_span!(SELECT, TABLE)))
            .await
            .with_context(|| format!("failed to fetch latest reps for profile_id={profile_id} and movement_id={movement_id}"))
            .map_err(into_log_server_error!())
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateReps {
    pub profile_id: Uuid,
    pub movement_id: Uuid,
    #[validate(range(min = 0))]
    pub amount: Option<i32>,
}

impl CreateReps {
    pub async fn insert_one(
        self,
        executor: impl Executor<'_, Database = DB>,
    ) -> OperationResult<Reps> {
        sqlx::query_as::<_, (i64, NaiveDateTime)>(formatcp!(
            "{INSERT_INTO} {TABLE} (profile_id, movement_id, amount) VALUES ($1, $2, $3) RETURNING id, timestamp",
        ))
        .bind(self.profile_id)
        .bind(self.movement_id)
        .bind(self.amount)
        .fetch_one(executor.instrument_executor(db_span!(INSERT_INTO, TABLE)))
        .await
        .map_err(|e| handle_error(e, || "failed to insert a new rep record"))
        .map(|(id, timestamp)| Reps {
            id,
            profile_id: self.profile_id,
            movement_id: self.movement_id,
            amount: self.amount,
            timestamp
        })
        .map_err(log_server_error!())
    }
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateReps {
    #[schema(value_type = String, format = Int64)]
    #[serde_as(as = "DisplayFromStr")]
    pub id: i64,
    #[validate(range(min = 0))]
    pub amount: Option<i32>,
}

impl UpdateReps {
    pub async fn update_one(
        self,
        executor: impl Executor<'_, Database = DB>,
    ) -> OperationResult<Option<Reps>> {
        sqlx::query_as::<_, Reps>(formatcp!(
            "{UPDATE} {TABLE} SET amount = $1 WHERE id = $2 RETURNING *"
        ))
        .bind(self.amount)
        .bind(self.id)
        .fetch_optional(executor.instrument_executor(db_span!(UPDATE, TABLE)))
        .await
        .map_err(|e| {
            handle_error(e, || {
                format!("failed to update reps with id={id}", id = self.id)
            })
        })
        .map_err(log_server_error!())
    }
}

pub async fn delete_latest_reps(
    profile_id: Uuid,
    movement_id: Uuid,
    executor: impl Executor<'_, Database = DB>,
) -> OperationResult<Option<i64>> {
    sqlx::query_as::<_, (i64,)>(formatcp!(
        "{DELETE_FROM} {TABLE} WHERE id = any(
            array(SELECT id FROM {TABLE} WHERE movement_id = $1 AND profile_id = $2 ORDER BY timestamp DESC LIMIT 1)
        ) RETURNING id"
    ))
    .bind(movement_id)
    .bind(profile_id)
    .fetch_optional(executor.instrument_executor(db_span!(DELETE_FROM, TABLE)))
    .await
    .map(|res| res.map(|(id,)| id))
    .context("failed to delete latest reps")
    .map_err(into_log_server_error!())
}
