use std::fmt::{Debug, Display};

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub struct Error {
    pub status: StatusCode,
    pub error: anyhow::Error,
}

pub type Result<T> = core::result::Result<T, Error>;

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.error, f)
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {:?}", self.status, self.error)
    }
}

impl IntoResponse for Error {
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

impl<E> From<E> for Error
where
    E: Into<anyhow::Error>,
{
    #[inline]
    fn from(err: E) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            error: err.into(),
        }
    }
}

pub trait IntoResult<T> {
    fn into_result(self) -> Result<T>;
    fn into_status(self, status: StatusCode) -> Result<T>;
}

impl<T, E> IntoResult<T> for core::result::Result<T, E>
where
    E: Into<Error>,
{
    #[inline]
    fn into_result(self) -> Result<T> {
        self.map_err(Into::<Error>::into)
    }

    #[inline]
    fn into_status(self, status: StatusCode) -> Result<T> {
        self.map_err(|e| {
            let mut err = e.into();
            err.status = status;
            err
        })
    }
}

pub trait LogError {
    /// Log the error if appropriate
    fn log_error(self) -> Self;
}

pub trait IsLoggable {
    #[inline(always)]
    fn is_loggable(&self) -> bool {
        true
    }
}

impl<T, E> LogError for core::result::Result<T, E>
where
    E: IsLoggable + Debug,
{
    #[inline]
    fn log_error(self) -> Self {
        if let Err(error) = &self {
            if error.is_loggable() {
                tracing::error!("{:?}", error);
            }
        }
        self
    }
}

impl<E> IsLoggable for E where E: Into<anyhow::Error> {}

impl IsLoggable for Error {
    #[inline]
    fn is_loggable(&self) -> bool {
        self.status.is_server_error()
    }
}
