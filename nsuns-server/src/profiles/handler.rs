use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

use crate::{
    db::{commit_ok, transaction, Pool},
    response_transforms::{created, or_404},
    validation::ValidatedJson,
};

use super::model::{CreateProfile, Profile};

pub async fn profiles_index(State(pool): State<Pool>) -> impl IntoResponse {
    Profile::select_all(&pool).await.map(Json)
}

pub async fn get_profile(State(pool): State<Pool>, Path(id): Path<Uuid>) -> impl IntoResponse {
    Profile::select_one(&pool, &id)
        .await
        .map(or_404::<_, Json<_>>)
}

pub async fn create_profile(
    State(pool): State<Pool>,
    ValidatedJson(profile): ValidatedJson<CreateProfile>,
) -> impl IntoResponse {
    let mut tx = transaction(&pool).await?;
    let res = profile.create_one(&mut tx).await.map(Json).map(created);
    commit_ok(res, tx).await
}

pub async fn update_profile(
    State(pool): State<Pool>,
    ValidatedJson(profile): ValidatedJson<Profile>,
) -> impl IntoResponse {
    let mut tx = transaction(&pool).await?;
    let res = profile.update_one(&mut tx).await.map(Json);
    commit_ok(res, tx).await
}

pub async fn delete_profile(State(pool): State<Pool>, Path(id): Path<Uuid>) -> impl IntoResponse {
    Profile::delete_one(&pool, &id)
        .await
        .map(or_404::<_, Json<_>>)
}
