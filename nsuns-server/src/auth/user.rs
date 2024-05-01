use axum::headers::authorization::Basic;
use chrono::{serde::ts_milliseconds, DateTime, Utc};
use const_format::formatcp;
use password_auth::{verify_password, ParseError, VerifyError};
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use sqlx::{Executor, FromRow};
use thiserror::Error;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    db::{
        tracing::{
            statements::{DELETE_FROM, INSERT_INTO, SELECT},
            InstrumentExecutor,
        },
        DB,
    },
    db_span,
};

use super::token::OwnerId;

const TABLE: &str = "users";

#[derive(Debug, Clone, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub username: String,
    password_hash: SecretString,
}

#[derive(Clone, Serialize, Deserialize, FromRow)]
struct UserRow {
    id: Uuid,
    owner_id: Uuid,
    username: String,
    password_hash: String,
}

#[derive(Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct UserInfo {
    pub id: Uuid,
    pub username: String,
}

#[derive(Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AnonymousInfo {
    #[schema(value_type = i64)]
    #[serde(with = "ts_milliseconds")]
    pub expiry_date: DateTime<Utc>,
}

#[derive(Clone, Serialize, Deserialize, ToSchema)]
#[serde(tag = "type")]
pub enum AgentInfo {
    #[serde(rename = "user")]
    User(UserInfo),
    #[serde(rename = "anonymous")]
    Anonymous(AnonymousInfo),
}

impl From<UserRow> for User {
    fn from(value: UserRow) -> Self {
        User {
            id: value.id,
            owner_id: value.owner_id,
            username: value.username,
            password_hash: value.password_hash.into(),
        }
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),

    #[error(transparent)]
    ParseError(#[from] ParseError),
}

/// Authenticate a user and return the user model
/// 
/// # Panics
/// Panics if the task spawned to compute the password hash is cancelled (indicates a bug)
pub async fn authenticate(
    executor: impl Executor<'_, Database = DB>,
    auth: Basic,
) -> Result<Option<User>, Error> {
    let user: Option<User> =
        sqlx::query_as::<_, UserRow>(formatcp!("{SELECT} * FROM {TABLE} WHERE username = $1"))
            .bind(auth.username())
            .fetch_optional(executor.instrument_executor(db_span!(SELECT, TABLE)))
            .await?
            .map(Into::into);

    if let Some(user) = user {
        tokio::task::spawn_blocking(move || {
            match verify_password(auth.password(), user.password_hash.expose_secret()) {
                Ok(()) => Ok(Some(user)),
                Err(VerifyError::Parse(e)) => Err(e.into()),
                Err(VerifyError::PasswordInvalid) => Ok(None),
            }
        })
        .await
        .expect("password hashing is not cancellable (this is a bug)")
    } else {
        Ok(None)
    }
}

pub async fn select_user_info_by_owner_id(
    owner_id: OwnerId,
    executor: impl Executor<'_, Database = DB>,
) -> Result<Option<UserInfo>, Error> {
    sqlx::query_as::<_, UserInfo>(formatcp!(
        "{SELECT} id, username FROM users WHERE owner_id = $1"
    ))
    .bind(owner_id)
    .fetch_optional(executor.instrument_executor(db_span!(SELECT, "users")))
    .await
    .map_err(Into::into)
}

pub async fn select_owner_expiry(
    owner_id: OwnerId,
    executor: impl Executor<'_, Database = DB>,
) -> Result<Option<DateTime<Utc>>, Error> {
    sqlx::query_as::<_, (DateTime<Utc>,)>(formatcp!(
        "{SELECT} expiry_date FROM owners WHERE id = $1"
    ))
    .bind(owner_id)
    .fetch_optional(executor.instrument_executor(db_span!(SELECT, "owners")))
    .await
    .map(|opt| opt.map(|exp| exp.0))
    .map_err(Into::into)
}

pub async fn create_anonymous_user(
    executor: impl Executor<'_, Database = DB>,
    expiry_date: DateTime<Utc>,
) -> Result<OwnerId, Error> {
    let id = sqlx::query_as::<_, (OwnerId,)>(formatcp!(
        "{INSERT_INTO} owners (expiry_date) VALUES ($1) RETURNING id"
    ))
    .bind(expiry_date)
    .fetch_one(executor.instrument_executor(db_span!(INSERT_INTO, "owners")))
    .await?
    .0;

    Ok(id)
}

pub async fn delete_owner(
    executor: impl Executor<'_, Database = DB>,
    id: OwnerId,
) -> Result<(), Error> {
    sqlx::query(formatcp!("{DELETE_FROM} owners WHERE id = $1"))
        .bind(id)
        .execute(executor.instrument_executor(db_span!(DELETE_FROM, "owners")))
        .await?;

    Ok(())
}
