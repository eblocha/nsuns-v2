use axum::{extract::State, response::IntoResponse, Json};

use crate::{
    acquire,
    auth::token::OwnerId,
    db::Pool,
    response_transforms::{created, or_404},
    validation::ValidatedJson,
};

use super::model::{CreateMovement, Movement};

#[tracing::instrument(skip_all)]
pub async fn movements_index(State(pool): State<Pool>, owner_id: OwnerId) -> impl IntoResponse {
    let mut conn = acquire!(&pool).await?;
    Movement::select_all(owner_id, &mut *conn).await.map(Json)
}

#[tracing::instrument(skip_all)]
pub async fn create_movement(
    State(pool): State<Pool>,
    owner_id: OwnerId,
    ValidatedJson(movement): ValidatedJson<CreateMovement>,
) -> impl IntoResponse {
    let mut conn = acquire!(&pool).await?;
    movement
        .insert_one(owner_id, &mut *conn)
        .await
        .map(Json)
        .map(created)
}

#[tracing::instrument(skip_all)]
pub async fn update_movement(
    State(pool): State<Pool>,
    owner_id: OwnerId,
    ValidatedJson(movement): ValidatedJson<Movement>,
) -> impl IntoResponse {
    let mut conn = acquire!(&pool).await?;
    movement
        .update_one(owner_id, &mut *conn)
        .await
        .map(or_404::<_, Json<_>>)
}
