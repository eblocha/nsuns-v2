use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

use crate::{
    auth::token::OwnerId,
    db::{
        commit_ok,
        transaction::{acquire, transaction},
        Pool,
    },
    response_transforms::{created, no_content_or_404, or_404},
    validation::ValidatedJson,
};

use super::model::{delete_one, CreateSet, UpdateSet};

#[tracing::instrument(skip_all)]
pub async fn create_set(
    State(pool): State<Pool>,
    owner_id: OwnerId,
    ValidatedJson(set): ValidatedJson<CreateSet>,
) -> impl IntoResponse {
    let mut conn = acquire(&pool).await?;
    let mut tx = transaction(&mut *conn).await?;
    let res = set
        .insert_one(owner_id, &mut tx)
        .await
        .map(Json)
        .map(created);

    commit_ok(res, tx).await
}

#[tracing::instrument(skip_all)]
pub async fn update_set(
    State(pool): State<Pool>,
    owner_id: OwnerId,
    ValidatedJson(set): ValidatedJson<UpdateSet>,
) -> impl IntoResponse {
    let mut conn = acquire(&pool).await?;
    let mut tx = transaction(&mut *conn).await?;
    let res = set.update_one(owner_id, &mut tx)
        .await
        .map(or_404::<_, Json<_>>);

    commit_ok(res, tx).await
}

#[tracing::instrument(skip_all)]
pub async fn delete_set(
    State(pool): State<Pool>,
    Path(id): Path<Uuid>,
    owner_id: OwnerId,
) -> impl IntoResponse {
    let mut conn = acquire(&pool).await?;
    let mut tx = transaction(&mut *conn).await?;
    let res = delete_one(id, owner_id, &mut tx)
        .await
        .map(no_content_or_404);

    commit_ok(res, tx).await
}
