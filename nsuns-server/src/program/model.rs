use anyhow::Context;
use serde::{Deserialize, Serialize};
use sqlx::{Executor, Transaction};
use uuid::Uuid;
use validator::Validate;

use crate::{
    db::DB,
    error::{IntoResult, Result},
    sets::model::Set,
};

#[derive(Debug, Serialize, Clone, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Program {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub owner: Uuid,
}

impl Program {
    pub async fn select_all_for_profile(
        executor: impl Executor<'_, Database = DB>,
        owner: &Uuid,
    ) -> Result<Vec<Self>> {
        sqlx::query_as::<_, Self>("SELECT * FROM programs WHERE owner = $1 ORDER BY created_on")
            .bind(owner)
            .fetch_all(executor)
            .await
            .with_context(|| format!("failed to select program with owner id={}", owner))
            .into_result()
    }

    pub async fn select_one(
        id: Uuid,
        executor: impl Executor<'_, Database = DB>,
    ) -> Result<Option<Self>> {
        sqlx::query_as::<_, Self>("SELECT * from programs WHERE id = $1")
            .bind(id)
            .fetch_optional(executor)
            .await
            .with_context(|| format!("failed to fetch program with id={}", id))
            .into_result()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateProgram {
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(length(min = 1))]
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

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProgram {
    pub id: Uuid,
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(length(min = 1))]
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

pub async fn delete_one(id: Uuid, executor: impl Executor<'_, Database = DB>) -> Result<Option<Program>> {
    sqlx::query_as::<_, Program>("DELETE FROM programs WHERE id = $1 RETURNING *")
        .bind(id)
        .fetch_optional(executor)
        .await
        .with_context(|| format!("failed to delete program with id={}", id))
        .into_result()
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramSummary {
    pub program: Program,
    pub sets: Vec<Set>,
}

pub async fn gather_program_summary(
    id: Uuid,
    tx: &mut Transaction<'_, DB>,
) -> Result<Option<ProgramSummary>> {
    let program_opt = Program::select_one(id, &mut **tx).await?;

    if let Some(program) = program_opt {
        let sets = Set::select_for_program(program.id, &mut **tx).await?;
        Ok(Some(ProgramSummary { program, sets }))
    } else {
        Ok(None)
    }
}
