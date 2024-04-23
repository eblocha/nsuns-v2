use axum::headers::authorization::Basic;
use const_format::formatcp;
use password_auth::{verify_password, ParseError, VerifyError};
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use sqlx::{Executor, FromRow};
use thiserror::Error;
use uuid::Uuid;

use crate::{
    db::{
        tracing::{statements::SELECT, InstrumentExecutor},
        DB,
    },
    db_span,
};

const TABLE: &str = "users";

#[derive(Debug, Clone, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    password_hash: SecretString,
}

#[derive(Clone, Serialize, Deserialize, FromRow)]
struct UserRow {
    id: Uuid,
    username: String,
    password_hash: String,
}

impl From<UserRow> for User {
    fn from(value: UserRow) -> Self {
        User {
            id: value.id,
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
                Ok(_) => Ok(Some(user)),
                Err(VerifyError::Parse(e)) => Err(e.into()),
                Err(VerifyError::PasswordInvalid) => Ok(None),
            }
        })
        .await
        .expect("password hashing is not cancellable")
    } else {
        Ok(None)
    }
}
