use anyhow::Context;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Executor, Transaction};
use uuid::Uuid;

use crate::{
    db::DB,
    error::{IntoResult, Result},
    user::model::User,
};

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Program {
    pub id: i32,
    pub name: Option<String>,
    pub description: Option<String>,
    pub created_on: DateTime<Utc>,
    pub owner: Uuid,
}

impl Program {
    pub async fn select_all_for_user(
        executor: impl Executor<'_, Database = DB>,
        owner: &Uuid,
    ) -> Result<Vec<Self>> {
        sqlx::query_as::<_, Program>("SELECT * FROM programs WHERE owner = $1 ORDER BY created_on")
            .bind(owner)
            .fetch_all(executor)
            .await
            .with_context(|| format!("failed to select program with owner id={}", owner))
            .into_result()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserPrograms {
    pub default: Option<Program>,
    pub all: Vec<Program>,
}

impl UserPrograms {
    pub async fn get_user_programs(
        tx: &mut Transaction<'_, DB>,
        owner: &Uuid,
    ) -> Result<UserPrograms> {
        let default_id = User::get_default_program_id(&mut **tx, owner).await?;
        let all_programs = Program::select_all_for_user(&mut **tx, owner).await?;

        let mut default: Option<Program> = None;

        if let Some(default_id) = default_id {
            for program in all_programs.iter() {
                if program.id == default_id {
                    default = Some(program.clone());
                    break;
                }
            }
        }

        Ok(UserPrograms {
            default,
            all: all_programs,
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateProgram {
    pub name: Option<String>,
    pub description: Option<String>,
    pub owner: Uuid,
}

impl CreateProgram {
    pub async fn insert_one(self, executor: impl Executor<'_, Database = DB>) -> Result<Program> {
        sqlx::query_as::<_, Program>(
            "INSERT INTO programs (name, description, owner) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(self.name)
        .bind(self.description)
        .bind(self.owner)
        .fetch_one(executor)
        .await
        .with_context(|| "failed to create program")
        .into_result()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProgram {
    pub id: i32,
    pub name: Option<String>,
    pub description: Option<String>,
}

impl UpdateProgram {
    pub async fn update_one(
        self,
        executor: impl Executor<'_, Database = DB>,
    ) -> Result<Option<Program>> {
        sqlx::query_as::<_, Program>(
            "UPDATE programs SET name = $1, description = $2 WHERE id = $3 RETURNING *",
        )
        .bind(self.name)
        .bind(self.description)
        .bind(self.id)
        .fetch_optional(executor)
        .await
        .with_context(|| format!("failed to update program with id={}", self.id))
        .into_result()
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct ProgramId(pub i32);

impl ProgramId {
    pub async fn delete_one(
        self,
        executor: impl Executor<'_, Database = DB>,
    ) -> Result<Option<()>> {
        sqlx::query("DELETE FROM programs WHERE id = $1")
            .bind(self.0)
            .execute(executor)
            .await
            .with_context(|| format!("failed to delete program with id={}", self.0))
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
