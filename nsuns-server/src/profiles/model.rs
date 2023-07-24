use anyhow::Context;
use serde::{Deserialize, Serialize};
use sqlx::{Executor, Transaction};
use uuid::Uuid;
use validator::Validate;

use crate::{
    db::DB,
    error::{IntoResult, Result},
};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub id: Uuid,
    #[validate(length(min = 1))]
    pub name: String,
}

impl Profile {
    pub async fn select_one(
        executor: impl Executor<'_, Database = DB>,
        id: &Uuid,
    ) -> Result<Option<Self>> {
        sqlx::query_as::<_, Self>("SELECT * from profiles WHERE id = $1")
            .bind(id)
            .fetch_optional(executor)
            .await
            .with_context(|| format!("failed to fetch profile with id={}", id))
            .into_result()
    }

    pub async fn select_all(executor: impl Executor<'_, Database = DB>) -> Result<Vec<Profile>> {
        sqlx::query_as::<_, Self>("SELECT * FROM profiles")
            .fetch_all(executor)
            .await
            .with_context(|| "failed to select all profiles")
            .into_result()
    }

    pub async fn update_one(self, tx: &mut Transaction<'_, DB>) -> Result<Option<Self>> {
        sqlx::query("UPDATE profiles SET name = $1 WHERE id = $2")
            .bind(&self.name)
            .bind(self.id)
            .execute(&mut **tx)
            .await
            .with_context(|| format!("failed to update profile with id={}", self.id))
            .map(|result| {
                if result.rows_affected() == 0 {
                    None
                } else {
                    Some(self)
                }
            })
            .into_result()
    }

    pub async fn delete_one(
        executor: impl Executor<'_, Database = DB>,
        id: &Uuid,
    ) -> Result<Option<Profile>> {
        sqlx::query_as::<_, Profile>("DELETE FROM profiles WHERE id = $1 RETURNING *")
            .bind(id)
            .fetch_optional(executor)
            .await
            .with_context(|| format!("failed to delete profile with id={}", id))
            .into_result()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateProfile {
    #[validate(length(min = 1))]
    pub name: String,
}

impl CreateProfile {
    pub async fn create_one(self, tx: &mut Transaction<'_, DB>) -> Result<Profile> {
        sqlx::query_as::<_, Profile>("INSERT INTO profiles (name) VALUES ($1) RETURNING *")
            .bind(self.name)
            .fetch_one(&mut **tx)
            .await
            .with_context(|| "failed to create profile")
            .into_result()
    }
}
