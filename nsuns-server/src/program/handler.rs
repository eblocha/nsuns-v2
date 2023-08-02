use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use utoipa::IntoParams;
use uuid::Uuid;

use crate::{
    db::{commit_ok, transaction, Pool},
    error::LogError,
    util::{created, or_404},
    validation::ValidatedJson,
};

use super::model::{delete_one, gather_program_summary, CreateProgram, ProgramMeta, UpdateProgram};

#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
#[serde(rename_all = "camelCase")]
pub struct ProgramQuery {
    pub profile_id: Uuid,
}

pub async fn profile_programs(
    Query(params): Query<ProgramQuery>,
    State(pool): State<Pool>,
) -> impl IntoResponse {
    ProgramMeta::select_all_for_profile(&pool, &params.profile_id)
        .await
        .map(Json)
        .log_error()
}

pub async fn create_program(
    State(pool): State<Pool>,
    ValidatedJson(program): ValidatedJson<CreateProgram>,
) -> impl IntoResponse {
    program
        .insert_one(&pool)
        .await
        .map(Json)
        .map(created)
        .log_error()
}

pub async fn update_program(
    State(pool): State<Pool>,
    ValidatedJson(program): ValidatedJson<UpdateProgram>,
) -> impl IntoResponse {
    program
        .update_one(&pool)
        .await
        .map(or_404::<_, Json<_>>)
        .log_error()
}

pub async fn delete_program(State(pool): State<Pool>, Path(id): Path<Uuid>) -> impl IntoResponse {
    let mut tx = transaction(&pool).await.log_error()?;
    let res = delete_one(id, &mut tx).await.map(or_404::<_, Json<_>>);
    commit_ok(res, tx).await.log_error()
}

pub async fn program_summary(State(pool): State<Pool>, Path(id): Path<Uuid>) -> impl IntoResponse {
    let mut tx = transaction(&pool).await.log_error()?;
    let res = gather_program_summary(id, &mut tx)
        .await
        .map(or_404::<_, Json<_>>);
    commit_ok(res, tx).await.log_error()
}
