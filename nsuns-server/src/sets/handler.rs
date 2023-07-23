use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

use crate::{
    db::{commit_ok, transaction, Pool},
    error::{IntoResult, LogError},
    util::{created, no_content_or_404, or_404},
};

use super::model::{delete_one, CreateSet, UpdateSet};

pub async fn create_set(State(pool): State<Pool>, Json(set): Json<CreateSet>) -> impl IntoResponse {
    let mut tx = transaction(&pool).await.log_error()?;
    let res = set
        .insert_one(&mut tx)
        .await
        .map(Json)
        .map(created)
        .into_result();

    commit_ok(res, tx).await.log_error()
}

pub async fn update_set(State(pool): State<Pool>, Json(set): Json<UpdateSet>) -> impl IntoResponse {
    set.update_one(&pool)
        .await
        .map(or_404::<_, Json<_>>)
        .log_error()
}

pub async fn delete_set(State(pool): State<Pool>, Path(id): Path<Uuid>) -> impl IntoResponse {
    let mut tx = transaction(&pool).await.log_error()?;
    let res = delete_one(id, &mut tx)
        .await
        .map(no_content_or_404)
        .into_result();

    commit_ok(res, tx).await.log_error()
}
