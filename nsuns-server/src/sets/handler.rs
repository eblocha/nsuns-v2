use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    db::{commit_ok, transaction, Pool},
    error::LogError,
    util::{created, or_404, no_content_or_404},
    validation::ValidatedJson,
};

use super::model::{delete_one, CreateSet, Day, UpdateSet};

pub async fn create_set(
    State(pool): State<Pool>,
    ValidatedJson(set): ValidatedJson<CreateSet>,
) -> impl IntoResponse {
    let mut tx = transaction(&pool).await.log_error()?;
    let res = set.insert_one(&mut tx).await.map(Json).map(created);

    commit_ok(res, tx).await.log_error()
}

pub async fn update_set(
    State(pool): State<Pool>,
    ValidatedJson(set): ValidatedJson<UpdateSet>,
) -> impl IntoResponse {
    set.update_one(&pool)
        .await
        .map(or_404::<_, Json<_>>)
        .log_error()
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DeleteSetMeta {
    program_id: Uuid,
    #[schema(value_type = u8)]
    day: Day,
}

pub async fn delete_set(
    State(pool): State<Pool>,
    Path(id): Path<Uuid>,
    Json(meta): Json<DeleteSetMeta>,
) -> impl IntoResponse {
    let mut tx = transaction(&pool).await.log_error()?;
    let res = delete_one(meta.program_id, meta.day, id, &mut tx)
        .await
        .map(no_content_or_404);

    commit_ok(res, tx).await.log_error()
}
