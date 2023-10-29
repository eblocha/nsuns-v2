use axum::{response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "status")]
pub enum Health {
    #[serde(rename = "OK")]
    Ok,
}

impl IntoResponse for Health {
    fn into_response(self) -> axum::response::Response {
        match self {
            Health::Ok => Json(self).into_response(),
        }
    }
}

pub async fn health_check() -> impl IntoResponse {
    Health::Ok
}
