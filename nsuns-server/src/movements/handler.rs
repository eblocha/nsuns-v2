use axum::{extract::State, response::IntoResponse, Json};

use crate::{db::Pool, error::LogError, util::or_404, validation::ValidatedJson};

use super::model::{CreateMovement, Movement};

pub async fn movements_index(State(pool): State<Pool>) -> impl IntoResponse {
    Movement::select_all(&pool).await.map(Json).log_error()
}

pub async fn create_movement(
    State(pool): State<Pool>,
    ValidatedJson(movement): ValidatedJson<CreateMovement>,
) -> impl IntoResponse {
    movement.insert_one(&pool).await.map(Json).log_error()
}

pub async fn update_movement(
    State(pool): State<Pool>,
    ValidatedJson(movement): ValidatedJson<Movement>,
) -> impl IntoResponse {
    movement
        .update_one(&pool)
        .await
        .map(or_404::<_, Json<_>>)
        .log_error()
}
