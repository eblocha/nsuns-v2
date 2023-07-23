use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

use crate::{
    db::{commit_ok, transaction, Pool},
    error::LogError,
    util::{created, or_404},
};

use super::model::{CreateProfile, Profile};

pub async fn profiles_index(State(pool): State<Pool>) -> impl IntoResponse {
    Profile::select_all(&pool).await.map(Json).log_error()
}

pub async fn get_profile(State(pool): State<Pool>, Path(id): Path<Uuid>) -> impl IntoResponse {
    Profile::select_one(&pool, &id)
        .await
        .map(or_404::<_, Json<_>>)
        .log_error()
}

pub async fn create_profile(
    State(pool): State<Pool>,
    Json(profile): Json<CreateProfile>,
) -> impl IntoResponse {
    let mut tx = transaction(&pool).await.log_error()?;
    let res = profile.create_one(&mut tx).await.map(Json).map(created);
    commit_ok(res, tx).await.log_error()
}

pub async fn update_profile(
    State(pool): State<Pool>,
    Json(profile): Json<Profile>,
) -> impl IntoResponse {
    let mut tx = transaction(&pool).await.log_error()?;
    let res = profile.update_one(&mut tx).await.map(Json);
    commit_ok(res, tx).await.log_error()
}

pub async fn delete_profile(State(pool): State<Pool>, Path(id): Path<Uuid>) -> impl IntoResponse {
    Profile::delete_one(&pool, &id)
        .await
        .map(or_404::<_, Json<_>>)
        .log_error()
}
