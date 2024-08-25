use axum::{extract::Request, middleware::Next, response::IntoResponse, Json};
use serde::Serialize;

use super::StoredErrorMessage;

#[derive(Debug, Serialize)]
struct ErrorResponse {
    path: String,
    message: String,
    status: u16,
}

/// Converts `ErrorWithStatus` responses (which store the message in response extensions)
/// into a json-serialized response, with extra metadata.
pub async fn json_errors(req: Request, next: Next) -> impl IntoResponse {
    let path = req.uri().path().to_owned();

    let mut response = next.run(req).await;

    let status = response.status();

    if let Some(StoredErrorMessage(message)) = response.extensions_mut().remove() {
        (
            status,
            Json(ErrorResponse {
                path,
                message,
                status: status.as_u16(),
            }),
        )
            .into_response()
    } else {
        response
    }
}
