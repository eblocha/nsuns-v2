use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    db::{commit_ok, transaction, Pool},
    error::{IntoResult, LogError},
    util::{created, no_content_or_404, or_404},
};

use super::model::{
    delete_one, gather_program_summary, CreateProgram, UpdateProgram, UserPrograms,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramQuery {
    pub user_id: Uuid,
}

pub async fn user_programs(
    Query(params): Query<ProgramQuery>,
    State(pool): State<Pool>,
) -> impl IntoResponse {
    let mut tx = transaction(&pool).await.log_error()?;
    let res = UserPrograms::get_user_programs(&mut tx, &params.user_id)
        .await
        .map(Json);

    commit_ok(res, tx).await.log_error()
}

pub async fn create_program(
    State(pool): State<Pool>,
    Json(program): Json<CreateProgram>,
) -> impl IntoResponse {
    program
        .insert_one(&pool)
        .await
        .map(Json)
        .map(created)
        .into_result()
        .log_error()
}

pub async fn update_program(
    State(pool): State<Pool>,
    Json(program): Json<UpdateProgram>,
) -> impl IntoResponse {
    program
        .update_one(&pool)
        .await
        .map(or_404::<_, Json<_>>)
        .into_result()
        .log_error()
}

pub async fn delete_program(State(pool): State<Pool>, Path(id): Path<i32>) -> impl IntoResponse {
    delete_one(id, &pool)
        .await
        .map(no_content_or_404)
        .into_result()
        .log_error()
}

pub async fn program_summary(State(pool): State<Pool>, Path(id): Path<i32>) -> impl IntoResponse {
    let mut tx = transaction(&pool).await.log_error()?;
    let res = gather_program_summary(id, &mut tx)
        .await
        .map(or_404::<_, Json<_>>)
        .into_result();
    commit_ok(res, tx).await.log_error()
}
