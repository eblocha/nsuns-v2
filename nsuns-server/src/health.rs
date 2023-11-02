use axum::{response::IntoResponse, Json};
use http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "status")]
pub enum Health {
    #[serde(rename = "OK")]
    Ok,
}

impl Health {
    fn status_code(&self) -> StatusCode {
        match self {
            Health::Ok => StatusCode::OK,
        }
    }
}

impl IntoResponse for Health {
    fn into_response(self) -> axum::response::Response {
        (self.status_code(), Json(self)).into_response()
    }
}

pub async fn health_check() -> impl IntoResponse {
    Health::Ok
}
