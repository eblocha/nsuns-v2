use anyhow::Context;
use serde::{Deserialize, Serialize};
use sqlx::Executor;
use uuid::Uuid;

use crate::{
    db::DB,
    error::{IntoResult, Result},
};

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Reps {
    pub id: i32,
    pub profile_id: Uuid,
    pub movement_id: i32,
    pub amount: i32,
}

impl Reps {
    pub async fn select_for_profile(
        profile_id: Uuid,
        executor: impl Executor<'_, Database = DB>,
    ) -> Result<Vec<Self>> {
        sqlx::query_as::<_, Self>("SELECT * FROM reps WHERE profile_id = $1 ORDER BY timestamp")
            .bind(profile_id)
            .fetch_all(executor)
            .await
            .with_context(|| format!("failed to select reps for profile_id={}", profile_id))
            .into_result()
    }

    pub async fn update_one(
        self,
        executor: impl Executor<'_, Database = DB>,
    ) -> Result<Option<Self>> {
        sqlx::query("UPDATE reps SET profile_id = $1, movement_id = $2, amount = $3 WHERE id = $4")
            .bind(self.profile_id)
            .bind(self.movement_id)
            .bind(self.amount)
            .execute(executor)
            .await
            .with_context(|| format!("failed to update reps with id={}", self.id))
            .map(|result| {
                if result.rows_affected() == 0 {
                    None
                } else {
                    Some(self)
                }
            })
            .into_result()
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateReps {
    pub profile_id: Uuid,
    pub movement_id: i32,
    pub amount: i32,
}

impl CreateReps {
    pub async fn insert_one(self, executor: impl Executor<'_, Database = DB>) -> Result<Reps> {
        sqlx::query_as::<_, (i32,)>(
            "INSERT INTO maxes (profile_id, movement_id, amount) VALUES ($1, $2, $3) RETURNING id",
        )
        .bind(self.profile_id)
        .bind(self.movement_id)
        .bind(self.amount)
        .fetch_one(executor)
        .await
        .with_context(|| "failed to insert a new rep record")
        .map(|(id,)| Reps {
            id,
            profile_id: self.profile_id,
            movement_id: self.movement_id,
            amount: self.amount,
        })
        .into_result()
    }
}