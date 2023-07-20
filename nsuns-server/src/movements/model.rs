use anyhow::Context;
use serde::{Deserialize, Serialize};
use sqlx::Executor;

use crate::{
    db::DB,
    error::{IntoResult, Result},
};

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Movement {
    pub id: i32,
    pub name: String,
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
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateMovement {
    pub name: String,
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
