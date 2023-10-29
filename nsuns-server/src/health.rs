use axum::{response::IntoResponse, Json};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "status")]
pub enum Health {
    #[serde(rename = "OK")]
    Ok,
}

pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, Json(Health::Ok))
}
