use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

use crate::{
    db::{commit_ok, transaction::{acquire, transaction}, Pool},
    response_transforms::{created, no_content_or_404, or_404},
    validation::ValidatedJson,
};

use super::model::{delete_one, CreateSet, UpdateSet};

pub async fn create_set(
    State(pool): State<Pool>,
    ValidatedJson(set): ValidatedJson<CreateSet>,
) -> impl IntoResponse {
    let mut conn = acquire(&pool).await?;
    let mut tx = transaction(&mut conn).await?;
    let res = set.insert_one(&mut tx).await.map(Json).map(created);

    commit_ok(res, tx).await
}

pub async fn update_set(
    State(pool): State<Pool>,
    ValidatedJson(set): ValidatedJson<UpdateSet>,
) -> impl IntoResponse {
    set.update_one(&pool).await.map(or_404::<_, Json<_>>)
}

pub async fn delete_set(State(pool): State<Pool>, Path(id): Path<Uuid>) -> impl IntoResponse {
    let mut conn = acquire(&pool).await?;
    let mut tx = transaction(&mut conn).await?;
    let res = delete_one(id, &mut tx).await.map(no_content_or_404);

    commit_ok(res, tx).await
}
