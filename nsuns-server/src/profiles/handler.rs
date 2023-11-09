use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

use crate::{
    db::{
        commit_ok,
        transaction::{acquire, transaction},
        Pool,
    },
    response_transforms::{created, or_404},
    validation::ValidatedJson,
};

use super::model::{CreateProfile, Profile};

#[tracing::instrument(skip_all)]
pub async fn profiles_index(State(pool): State<Pool>) -> impl IntoResponse {
    let mut conn = acquire(&pool).await?;
    Profile::select_all(&mut *conn).await.map(Json)
}

#[tracing::instrument(skip_all)]
pub async fn get_profile(State(pool): State<Pool>, Path(id): Path<Uuid>) -> impl IntoResponse {
    let mut conn = acquire(&pool).await?;
    Profile::select_one(&mut *conn, &id)
        .await
        .map(or_404::<_, Json<_>>)
}

#[tracing::instrument(skip_all)]
pub async fn create_profile(
    State(pool): State<Pool>,
    ValidatedJson(profile): ValidatedJson<CreateProfile>,
) -> impl IntoResponse {
    let mut conn = acquire(&pool).await?;
    let mut tx = transaction(&mut *conn).await?;
    let res = profile.create_one(&mut tx).await.map(Json).map(created);
    commit_ok(res, tx).await
}

#[tracing::instrument(skip_all)]
pub async fn update_profile(
    State(pool): State<Pool>,
    ValidatedJson(profile): ValidatedJson<Profile>,
) -> impl IntoResponse {
    let mut conn = acquire(&pool).await?;
    let mut tx = transaction(&mut *conn).await?;
    let res = profile.update_one(&mut tx).await.map(Json);
    commit_ok(res, tx).await
}

#[tracing::instrument(skip_all)]
pub async fn delete_profile(State(pool): State<Pool>, Path(id): Path<Uuid>) -> impl IntoResponse {
    let mut conn = acquire(&pool).await?;
    Profile::delete_one(&mut *conn, &id)
        .await
        .map(or_404::<_, Json<_>>)
}
