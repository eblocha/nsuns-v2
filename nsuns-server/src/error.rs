use std::fmt::{Debug, Display};

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

/// An error with a corresponding status code to be returned to the client.
/// By default, this status code will be 500 (internal server error).
#[derive(thiserror::Error)]
pub struct ErrorWithStatus<E> {
    pub status: StatusCode,
    #[source]
    pub error: E,
}

impl<E> From<E> for ErrorWithStatus<E> {
    fn from(error: E) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            error,
        }
    }
}

impl<E> IntoResponse for ErrorWithStatus<E>
where
    E: Display,
{
    fn into_response(self) -> Response {
        let message = if self.status.is_server_error() {
            self.status
                .canonical_reason()
                .unwrap_or("<unknown status code>")
                .to_string()
        } else {
            self.error.to_string()
        };

        (self.status, message).into_response()
    }
}

impl<E> Display for ErrorWithStatus<E>
where
    E: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.error, f)
    }
}

impl<E> Debug for ErrorWithStatus<E>
where
    E: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {:?}", self.status, self.error)
    }
}

/// Represents the result of an HTTP operation.
/// Contains a successful response, or an error with a status code.
pub type OperationResult<T, E = anyhow::Error> = core::result::Result<T, ErrorWithStatus<E>>;

pub trait LogError {
    /// Log the error if appropriate
    fn log_error(self) -> Self;
}

impl<T, E> LogError for OperationResult<T, E>
where
    E: Debug,
{
    fn log_error(self) -> Self {
        if let Err(error) = &self {
            if error.status.is_server_error() {
                tracing::error!("{error:?}");
            }
        }
        self
    }
}

impl<T> LogError for anyhow::Result<T> {
    fn log_error(self) -> Self {
        if let Err(error) = &self {
            tracing::error!("{error:?}");
        }
        self
    }
}
