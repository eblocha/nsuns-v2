use anyhow::Context;
use chrono::naive::serde::ts_milliseconds;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::Executor;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::{
    db::DB,
    error::{IntoResult, Result},
};

#[derive(Debug, Serialize, Clone, sqlx::FromRow, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Max {
    #[schema(value_type = String, format = Int64)]
    #[serde(with = "crate::serde_display")]
    pub id: i64,
    pub profile_id: Uuid,
    pub movement_id: Uuid,
    pub amount: f64,
    #[schema(value_type = i64)]
    #[serde(with = "ts_milliseconds")]
    pub timestamp: NaiveDateTime,
}

impl Max {
    pub async fn select_for_profile(
        profile_id: Uuid,
        executor: impl Executor<'_, Database = DB>,
    ) -> Result<Vec<Self>> {
        sqlx::query_as::<_, Self>("SELECT * FROM maxes WHERE profile_id = $1 ORDER BY timestamp")
            .bind(profile_id)
            .fetch_all(executor)
            .await
            .with_context(|| format!("failed to select maxes for profile_id={}", profile_id))
            .into_result()
    }

    pub async fn update_one(
        self,
        executor: impl Executor<'_, Database = DB>,
    ) -> Result<Option<Self>> {
        sqlx::query("UPDATE maxes SET profile_id = $1, movement_id = $2, amount = $3 WHERE id = $4")
            .bind(self.profile_id)
            .bind(self.movement_id)
            .bind(self.amount)
            .bind(self.id)
            .execute(executor)
            .await
            .with_context(|| format!("failed to update max with id={}", self.id))
            .map(|result| {
                if result.rows_affected() == 0 {
                    None
                } else {
                    Some(self)
                }
            })
            .into_result()
    }

    pub async fn select_latest(
        movement_id: Uuid,
        profile_id: Uuid,
        executor: impl Executor<'_, Database = DB>,
    ) -> Result<Option<Self>> {
        sqlx::query_as::<_, Self>("SELECT * FROM maxes WHERE movement_id = $1 AND profile_id = $2 ORDER BY timestamp DESC LIMIT 1")
            .bind(movement_id)
            .bind(profile_id)
            .fetch_optional(executor)
            .await
            .with_context(|| format!("failed to fetch latest max for profile_id={} and movement_id={}", profile_id, movement_id))
            .into_result()
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
    pub async fn insert_one(self, executor: impl Executor<'_, Database = DB>) -> Result<Max> {
        sqlx::query_as::<_, (i64, NaiveDateTime)>(
            "INSERT INTO maxes (profile_id, movement_id, amount) VALUES ($1, $2, $3) RETURNING id, timestamp",
        )
        .bind(self.profile_id)
        .bind(self.movement_id)
        .bind(self.amount)
        .fetch_one(executor)
        .await
        .with_context(|| "failed to insert a new max")
        .map(|(id, timestamp)| Max {
            id,
            profile_id: self.profile_id,
            movement_id: self.movement_id,
            amount: self.amount,
            timestamp
        })
        .into_result()
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateMax {
    #[schema(value_type = String, format = Int64)]
    #[serde(with = "crate::serde_display")]
    pub id: i64,
    #[validate(range(min = 0))]
    pub amount: f64,
}

impl UpdateMax {
    pub async fn update_one(
        self,
        executor: impl Executor<'_, Database = DB>,
    ) -> Result<Option<Max>> {
        sqlx::query_as::<_, Max>("UPDATE maxes SET amount = $1 WHERE id = $2 RETURNING *")
            .bind(self.amount)
            .bind(self.id)
            .fetch_optional(executor)
            .await
            .with_context(|| format!("failed to update max with id={}", self.id))
            .into_result()
    }
}

pub async fn delete_latest_maxes(
    profile_id: Uuid,
    movement_id: Uuid,
    executor: impl Executor<'_, Database = DB>,
) -> Result<Option<i64>> {
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
    .into_result()
}
