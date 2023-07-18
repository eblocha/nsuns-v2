use anyhow::anyhow;
use axum::http::StatusCode;
use sqlx::Executor;
use thiserror::Error;

use crate::{
    db::DB,
    error::{Error, IntoResult},
};

#[derive(Debug, Error)]
pub enum UserValidationError<E> {
    /// The username is not unique
    UsernameTaken,
    /// Another error while validating
    Other(#[from] E),
}

impl<E> From<UserValidationError<E>> for Error
where
    E: Into<Error>,
{
    fn from(value: UserValidationError<E>) -> Self {
        match value {
            UserValidationError::UsernameTaken => Error {
                status: StatusCode::CONFLICT,
                error: anyhow!("the given username is taken"),
            },
            UserValidationError::Other(e) => e.into(),
        }
    }
}

pub async fn validate_user(
    username: &str,
    executor: impl Executor<'_, Database = DB>,
) -> Result<(), UserValidationError<Error>> {
    let is_taken =
        sqlx::query_as::<_, (bool,)>("SELECT EXISTS(SELECT 1 FROM users WHERE username = $1)")
            .bind(username)
            .fetch_one(executor)
            .await
            .into_result()?
            .0;

    return if is_taken {
        Err(UserValidationError::UsernameTaken)
    } else {
        Ok(())
    };
}
