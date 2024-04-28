use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use utoipa::IntoParams;
use uuid::Uuid;

use crate::{
    auth::token::OwnerId,
    db::{
        commit_ok,
        transaction::{acquire, transaction},
        Pool,
    },
    error::extract::WithErrorRejection,
    response_transforms::{created, or_404},
    validation::ValidatedJson,
};

use super::model::{
    delete_one, gather_program_summary, CreateProgram, ProgramMeta, ReorderSets, UpdateProgram,
};

#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
#[serde(rename_all = "camelCase")]
pub struct ProgramQuery {
    pub profile_id: Uuid,
}

#[tracing::instrument(skip_all)]
pub async fn profile_programs(
    State(pool): State<Pool>,
    WithErrorRejection(Query(params)): WithErrorRejection<Query<ProgramQuery>>,
    owner_id: OwnerId,
) -> impl IntoResponse {
    let mut conn = acquire(&pool).await?;
    ProgramMeta::select_all_for_profile(params.profile_id, owner_id, &mut *conn)
        .await
        .map(Json)
}

#[tracing::instrument(skip_all)]
pub async fn create_program(
    State(pool): State<Pool>,
    owner_id: OwnerId,
    ValidatedJson(program): ValidatedJson<CreateProgram>,
) -> impl IntoResponse {
    let mut conn = acquire(&pool).await?;
    let mut tx = transaction(&mut *conn).await?;

    let res = program
        .insert_one(owner_id, &mut tx)
        .await
        .map(Json)
        .map(created);

    commit_ok(res, tx).await
}

#[tracing::instrument(skip_all)]
pub async fn update_program(
    State(pool): State<Pool>,
    owner_id: OwnerId,
    ValidatedJson(program): ValidatedJson<UpdateProgram>,
) -> impl IntoResponse {
    let mut conn = acquire(&pool).await?;
    program
        .update_one(owner_id, &mut *conn)
        .await
        .map(or_404::<_, Json<_>>)
}

#[tracing::instrument(skip_all)]
pub async fn reorder_sets(
    State(pool): State<Pool>,
    owner_id: OwnerId,
    ValidatedJson(reorder): ValidatedJson<ReorderSets>,
) -> impl IntoResponse {
    let mut conn = acquire(&pool).await?;
    let mut tx = transaction(&mut *conn).await?;
    let res = reorder
        .reorder(owner_id, &mut tx)
        .await
        .map(or_404::<_, Json<_>>);
    commit_ok(res, tx).await
}

#[tracing::instrument(skip_all)]
pub async fn delete_program(
    State(pool): State<Pool>,
    Path(id): Path<Uuid>,
    owner_id: OwnerId,
) -> impl IntoResponse {
    let mut conn = acquire(&pool).await?;
    delete_one(id, owner_id, &mut *conn)
        .await
        .map(or_404::<_, Json<_>>)
}

#[tracing::instrument(skip_all)]
pub async fn program_summary(
    State(pool): State<Pool>,
    Path(id): Path<Uuid>,
    owner_id: OwnerId,
) -> impl IntoResponse {
    let mut conn = acquire(&pool).await?;
    let mut tx = transaction(&mut *conn).await?;
    let res = gather_program_summary(id, owner_id, &mut tx)
        .await
        .map(or_404::<_, Json<_>>);
    commit_ok(res, tx).await
}
