#[doc(hidden)]
#[macro_export]
macro_rules! log_server_error_impl {
    ($error:ident) => {
        if $error.status.is_server_error() && $error.take() {
            tracing::error!("{:?}", $error.error);
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
        |error: $crate::error::ErrorWithStatus<_>| {
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
            let error: $crate::error::ErrorWithStatus<_> = error.into();
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
