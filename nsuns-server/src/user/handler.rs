use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

use crate::{
    db::{commit_ok, transaction, Pool},
    error::LogError,
    util::{no_content_or_404, or_404},
};

use super::model::{CreateUser, User, UsernameTaken};

pub async fn users_index(State(pool): State<Pool>) -> impl IntoResponse {
    User::get_users(&pool).await.map(Json).log_error()
}

pub async fn get_user(State(pool): State<Pool>, Path(id): Path<Uuid>) -> impl IntoResponse {
    User::select_one(&pool, &id)
        .await
        .map(or_404::<_, Json<_>>)
        .log_error()
}

pub async fn create_user(
    State(pool): State<Pool>,
    Json(user): Json<CreateUser>,
) -> impl IntoResponse {
    let mut tx = transaction(&pool).await.log_error()?;
    let res = user.create_one(&mut tx).await.map(Json);
    commit_ok(res, tx).await.log_error()
}

pub async fn update_user(State(pool): State<Pool>, Json(user): Json<User>) -> impl IntoResponse {
    let mut tx = transaction(&pool).await.log_error()?;
    let res = user.update_one(&mut tx).await.map(Json);
    commit_ok(res, tx).await.log_error()
}

pub async fn delete_user(State(pool): State<Pool>, Path(id): Path<Uuid>) -> impl IntoResponse {
    User::delete_one(&pool, &id)
        .await
        .map(no_content_or_404)
        .log_error()
}

pub async fn is_taken(State(pool): State<Pool>, Path(username): Path<String>) -> impl IntoResponse {
    UsernameTaken::is_username_taken(&pool, &username)
        .await
        .map(Json)
        .log_error()
}
