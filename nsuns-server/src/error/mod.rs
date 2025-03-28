pub mod extract;
pub mod macros;
pub mod middleware;

use std::{fmt::{Debug, Display}, sync::atomic::{AtomicBool, Ordering}};

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
    pub logged: AtomicBool,
}

impl<E> ErrorWithStatus<E> {
    pub fn new(status: StatusCode, error: E) -> Self {
        Self {
            status,
            error,
            logged: AtomicBool::new(false),
        }
    }

    #[doc(hidden)]
    pub fn take(&self) -> bool {
        !self.logged.swap(true, Ordering::Relaxed)
    }
}

impl<E> From<E> for ErrorWithStatus<E> {
    fn from(error: E) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            error,
            logged: AtomicBool::new(false),
        }
    }
}

#[derive(Debug, Clone)]
pub struct StoredErrorMessage(String);

impl<E> IntoResponse for &ErrorWithStatus<E>
where
    E: Display + Debug,
{
    fn into_response(self) -> Response {
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

        let mut response = self.status.into_response();

        // place the error message into response extensions so middleware can put it into a JSON response easily
        response
            .extensions_mut()
            .insert(StoredErrorMessage(message));

        response
    }
}

impl<E> IntoResponse for ErrorWithStatus<E>
where
    E: Display + Debug,
{
    fn into_response(self) -> Response {
        (&self).into_response()
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
