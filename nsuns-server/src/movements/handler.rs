use axum::{extract::State, response::IntoResponse, Json};

use crate::{db::Pool, error::LogError};

use super::model::{CreateMovement, Movement};

pub async fn movements_index(State(pool): State<Pool>) -> impl IntoResponse {
    Movement::select_all(&pool).await.map(Json).log_error()
}

pub async fn create_movement(
    State(pool): State<Pool>,
    Json(movement): Json<CreateMovement>,
) -> impl IntoResponse {
    movement.insert_one(&pool).await.map(Json).log_error()
}
