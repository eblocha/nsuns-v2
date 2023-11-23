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
    pub logged: bool,
}

impl<E> ErrorWithStatus<E> {
    pub fn new(status: StatusCode, error: E) -> Self {
        Self {
            status,
            error,
            logged: false,
        }
    }
}

impl<E> From<E> for ErrorWithStatus<E> {
    fn from(error: E) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            error,
            logged: false,
        }
    }
}

impl<E> IntoResponse for ErrorWithStatus<E>
where
    E: Display + Debug,
{
    fn into_response(mut self) -> Response {
        // log if it hasn't been logged
        crate::log_server_error_impl!(self);

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

#[doc(hidden)]
#[macro_export]
macro_rules! log_server_error_impl {
    ($error:ident) => {
        if $error.status.is_server_error() && !$error.logged {
            tracing::error!("{:?}", $error.error);
            $error.logged = true;
        }
    };
}

/// Create a closure that logs an `ErrorWithStatus` if it's a server error, then returns the error.
///
/// Useful for logging errors with `result.map_err(log_server_error!())`
#[macro_export]
macro_rules! log_server_error {
    () => {
        // this is a macro so the file/line numbers work correctly in error tracing
        |mut error: $crate::error::ErrorWithStatus<_>| {
            $crate::log_server_error_impl!(error);
            error
        }
    };
}

/// Create a closure that converts an error into an `ErrorWithStatus`, logs it if appropriate,
/// then returns the `ErrorWithStatus`.
///
/// Useful for logging errors with `result.map_err(into_log_server_error!())`
#[macro_export]
macro_rules! into_log_server_error {
    () => {
        // this is a macro so the file/line numbers work correctly in error tracing
        |error| {
            let mut error: $crate::error::ErrorWithStatus<_> = error.into();
            $crate::log_server_error_impl!(error);
            error
        }
    };
}

/// Create a closure that logs an error, then returns the error.
///
/// Useful for logging errors with `result.map_err(log_error!())`
#[macro_export]
macro_rules! log_error {
    () => {
        // this is a macro so the file/line numbers work correctly in error tracing
        |error| {
            tracing::error!("{error:?}");
            error
        }
    };
}
