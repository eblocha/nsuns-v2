use anyhow::Context;
use serde::{Deserialize, Serialize};
use sqlx::Executor;
use uuid::Uuid;
use validator::Validate;

use crate::{
    db::DB,
    error::{IntoResult, Result},
};

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Movement {
    pub id: Uuid,
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(length(min = 1))]
    pub description: Option<String>,
}

impl Movement {
    pub async fn select_all(executor: impl Executor<'_, Database = DB>) -> Result<Vec<Self>> {
        sqlx::query_as::<_, Self>("SELECT * FROM movements")
            .fetch_all(executor)
            .await
            .with_context(|| "failed to select movements")
            .into_result()
    }

    pub async fn update_one(
        self,
        executor: impl Executor<'_, Database = DB>,
    ) -> Result<Option<Self>> {
        sqlx::query("UPDATE movements SET name = $1, description = $2 WHERE id = $3")
            .bind(&self.name)
            .bind(self.description.as_ref())
            .bind(self.id)
            .execute(executor)
            .await
            .with_context(|| format!("failed to update movement with id={}", self.id))
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

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateMovement {
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(length(min = 1))]
    pub description: Option<String>,
}

impl CreateMovement {
    pub async fn insert_one(self, executor: impl Executor<'_, Database = DB>) -> Result<Movement> {
        sqlx::query_as::<_, Movement>(
            "INSERT INTO movements (name, description) VALUES ($1, $2) RETURNING *",
        )
        .bind(&self.name)
        .bind(self.description.as_ref())
        .fetch_one(executor)
        .await
        .with_context(|| "failed to insert new movement")
        .into_result()
    }
}
