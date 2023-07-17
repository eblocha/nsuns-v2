use anyhow::Context;
use serde::{Deserialize, Serialize};
use sqlx::Executor;
use uuid::Uuid;

use crate::{
    db::DB,
    error::{IntoResult, Result},
};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub name: Option<String>,
    pub default_program: Option<i32>,
}

impl User {
    pub async fn get_default_program_id(
        executor: impl Executor<'_, Database = DB>,
        id: &Uuid,
    ) -> Result<Option<i32>> {
        sqlx::query_as::<_, (Option<i32>,)>("SELECT default_program FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(executor)
            .await
            .map(|opt| opt.and_then(|(id,)| id))
            .into_result()
    }

    pub async fn get_users(executor: impl Executor<'_, Database = DB>) -> Result<Vec<User>> {
        sqlx::query_as::<_, Self>("SELECT * FROM users")
            .fetch_all(executor)
            .await
            .into_result()
    }

    pub async fn update_one(
        self,
        executor: impl Executor<'_, Database = DB>,
    ) -> Result<Option<Self>> {
        sqlx::query_as::<_, Self>(
            "UPDATE users SET username = $1, name = $2, default_program = $3 WHERE id = $4 RETURNING *",
        )
        .bind(self.username)
        .bind(self.name)
        .bind(self.default_program)
        .bind(self.id)
        .fetch_optional(executor)
        .await
        .into_result()
    }

    pub async fn delete_one(
        executor: impl Executor<'_, Database = DB>,
        id: &Uuid,
    ) -> Result<Option<()>> {
        sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id)
            .execute(executor)
            .await
            .with_context(|| format!("failed to delete user with id={}", id))
            .map(|result| {
                if result.rows_affected() == 0 {
                    None
                } else {
                    Some(())
                }
            })
            .into_result()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UsernameTaken {
    pub taken: bool,
}

impl UsernameTaken {
    pub async fn is_username_taken(
        executor: impl Executor<'_, Database = DB>,
        username: &str,
    ) -> Result<UsernameTaken> {
        sqlx::query_as::<_, (bool,)>("SELECT EXISTS(SELECT 1 FROM users WHERE username = $1)")
            .bind(username)
            .fetch_one(executor)
            .await
            .with_context(|| format!("failed to determine if username={} is taken", username))
            .map(|(taken,)| UsernameTaken { taken })
            .into_result()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateUser {
    pub username: String,
    pub name: Option<String>,
}

impl CreateUser {
    pub async fn create_one(self, executor: impl Executor<'_, Database = DB>) -> Result<User> {
        sqlx::query_as::<_, User>("INSERT INTO users (username, name) VALUES ($1, $2) RETURNING *")
            .bind(self.username)
            .bind(self.name)
            .fetch_one(executor)
            .await
            .into_result()
    }
}
