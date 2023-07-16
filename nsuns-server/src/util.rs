use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use tokio::signal;

/// Set the status code to 201
pub fn created<T>(body: T) -> (StatusCode, T)
where
    T: IntoResponse,
{
    (StatusCode::CREATED, body)
}

/// Set status code to 404 if the value is None,
/// or convert the value to a Response if Some
pub fn or_404<T, E>(opt: Option<T>) -> Response
where
    E: IntoResponse + From<T>,
{
    match opt {
        Some(data) => E::from(data).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

/// Set status code to 404 if the value is None,
/// or return a no-content status if Some
pub fn no_content_or_404<T>(opt: Option<T>) -> StatusCode {
    match opt {
        Some(_) => StatusCode::NO_CONTENT,
        None => StatusCode::NOT_FOUND,
    }
}

pub async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("signal received, starting graceful shutdown");
}
