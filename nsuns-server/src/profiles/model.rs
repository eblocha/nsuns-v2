use anyhow::Context;
use const_format::formatcp;
use serde::{Deserialize, Serialize};
use sqlx::{Executor, Transaction};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::{
    assert_all_owner, assert_owner,
    auth::token::OwnerId,
    db::{
        tracing::{
            statements::{DELETE_FROM, INSERT_INTO, SELECT, UPDATE},
            InstrumentExecutor,
        },
        DB,
    },
    db_span,
    error::OperationResult,
    into_log_server_error,
};

const TABLE: &str = "profiles";

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub id: Uuid,
    #[validate(length(min = 1))]
    pub name: String,
}

impl Profile {
    pub async fn select_one(
        id: Uuid,
        owner_id: OwnerId,
        executor: impl Executor<'_, Database = DB>,
    ) -> OperationResult<Option<Self>> {
        sqlx::query_as::<_, Self>(formatcp!(
            "{SELECT} * from {TABLE} WHERE id = $1 AND owner_id = $2"
        ))
        .bind(id)
        .bind(owner_id)
        .fetch_optional(executor.instrument_executor(db_span!(SELECT, TABLE)))
        .await
        .with_context(|| format!("failed to fetch profile with id={id}"))
        .map_err(into_log_server_error!())
    }

    pub async fn select_all(
        owner_id: OwnerId,
        executor: impl Executor<'_, Database = DB>,
    ) -> OperationResult<Vec<Profile>> {
        sqlx::query_as::<_, Self>(formatcp!("{SELECT} * FROM {TABLE} WHERE owner_id = $1"))
            .bind(owner_id)
            .fetch_all(executor.instrument_executor(db_span!(SELECT, TABLE)))
            .await
            .context("failed to select all profiles")
            .map_err(into_log_server_error!())
    }

    pub async fn update_one(
        self,
        owner_id: OwnerId,
        tx: &mut Transaction<'_, DB>,
    ) -> OperationResult<Option<Self>> {
        sqlx::query(formatcp!(
            "{UPDATE} {TABLE} SET name = $1 WHERE id = $2 AND owner_id = $3"
        ))
        .bind(&self.name)
        .bind(self.id)
        .bind(owner_id)
        .execute((&mut **tx).instrument_executor(db_span!(UPDATE, TABLE)))
        .await
        .with_context(|| format!("failed to update profile with id={id}", id = self.id))
        .map(|result| {
            if result.rows_affected() == 0 {
                None
            } else {
                Some(self)
            }
        })
        .map_err(into_log_server_error!())
    }

    pub async fn delete_one(
        id: Uuid,
        owner_id: OwnerId,
        executor: impl Executor<'_, Database = DB>,
    ) -> OperationResult<Option<Profile>> {
        sqlx::query_as::<_, Profile>(formatcp!(
            "{DELETE_FROM} {TABLE} WHERE id = $1 AND owner_id = $2 RETURNING *"
        ))
        .bind(id)
        .bind(owner_id)
        .fetch_optional(executor.instrument_executor(db_span!(DELETE_FROM, TABLE)))
        .await
        .with_context(|| format!("failed to delete profile with id={id}"))
        .map_err(into_log_server_error!())
    }

    pub async fn assert_owner(
        id: Uuid,
        owner_id: OwnerId,
        executor: impl Executor<'_, Database = DB>,
    ) -> OperationResult<()> {
        assert_owner!(TABLE, "profile", id, owner_id, executor)
    }

    pub async fn assert_all_owner(
        ids: &[Uuid],
        owner_id: OwnerId,
        executor: impl Executor<'_, Database = DB>,
    ) -> OperationResult<()> {
        assert_all_owner!(TABLE, "profile", ids, owner_id, executor)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateProfile {
    #[validate(length(min = 1))]
    pub name: String,
}

impl CreateProfile {
    pub async fn create_one(
        self,
        owner_id: OwnerId,
        tx: &mut Transaction<'_, DB>,
    ) -> OperationResult<Profile> {
        sqlx::query_as::<_, Profile>(formatcp!(
            "{INSERT_INTO} {TABLE} (name, owner_id) VALUES ($1, $2) RETURNING *"
        ))
        .bind(self.name)
        .bind(owner_id)
        .fetch_one((&mut **tx).instrument_executor(db_span!(INSERT_INTO, TABLE)))
        .await
        .context("failed to create profile")
        .map_err(into_log_server_error!())
    }
}
