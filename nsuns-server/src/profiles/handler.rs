use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

use crate::{
    acquire,
    auth::token::OwnerId,
    db::{commit_ok, Pool},
    response_transforms::{created, or_404},
    transaction,
    validation::ValidatedJson,
};

use super::model::{CreateProfile, Profile};

#[tracing::instrument(skip_all)]
pub async fn profiles_index(State(pool): State<Pool>, owner_id: OwnerId) -> impl IntoResponse {
    let mut conn = acquire!(&pool).await?;
    Profile::select_all(owner_id, &mut *conn).await.map(Json)
}

#[tracing::instrument(skip_all)]
pub async fn get_profile(
    State(pool): State<Pool>,
    Path(id): Path<Uuid>,
    owner_id: OwnerId,
) -> impl IntoResponse {
    let mut conn = acquire!(&pool).await?;
    Profile::select_one(id, owner_id, &mut *conn)
        .await
        .map(or_404::<_, Json<_>>)
}

#[tracing::instrument(skip_all)]
pub async fn create_profile(
    State(pool): State<Pool>,
    owner_id: OwnerId,
    ValidatedJson(profile): ValidatedJson<CreateProfile>,
) -> impl IntoResponse {
    let mut tx = transaction!(&pool).await?;
    let res = profile
        .create_one(owner_id, &mut tx)
        .await
        .map(Json)
        .map(created);
    commit_ok(res, tx).await
}

#[tracing::instrument(skip_all)]
pub async fn update_profile(
    State(pool): State<Pool>,
    owner_id: OwnerId,
    ValidatedJson(profile): ValidatedJson<Profile>,
) -> impl IntoResponse {
    let mut tx = transaction!(&pool).await?;
    let res = profile.update_one(owner_id, &mut tx).await.map(Json);
    commit_ok(res, tx).await
}

#[tracing::instrument(skip_all)]
pub async fn delete_profile(
    State(pool): State<Pool>,
    Path(id): Path<Uuid>,
    owner_id: OwnerId,
) -> impl IntoResponse {
    let mut conn = acquire!(&pool).await?;
    Profile::delete_one(id, owner_id, &mut *conn)
        .await
        .map(or_404::<_, Json<_>>)
}
