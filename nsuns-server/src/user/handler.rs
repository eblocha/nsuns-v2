use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

use crate::{
    db::Pool,
    error::{IntoResult, LogError},
    util::no_content_or_404,
};

use super::model::{CreateUser, User};

pub async fn users_index(State(pool): State<Pool>) -> impl IntoResponse {
    User::get_users(&pool)
        .await
        .map(Json)
        .into_result()
        .log_error()
}

pub async fn create_user(
    State(pool): State<Pool>,
    Json(user): Json<CreateUser>,
) -> impl IntoResponse {
    user.create_one(&pool)
        .await
        .map(Json)
        .into_result()
        .log_error()
}

pub async fn update_user(State(pool): State<Pool>, Json(user): Json<User>) -> impl IntoResponse {
    user.update_one(&pool)
        .await
        .map(Json)
        .into_result()
        .log_error()
}

pub async fn delete_user(State(pool): State<Pool>, Path(id): Path<Uuid>) -> impl IntoResponse {
    User::delete_one(&pool, &id)
        .await
        .map(no_content_or_404)
        .into_result()
        .log_error()
}
