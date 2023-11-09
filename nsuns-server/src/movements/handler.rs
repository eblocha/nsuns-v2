use axum::{extract::State, response::IntoResponse, Json};

use crate::{
    db::{acquire, Pool},
    response_transforms::or_404,
    validation::ValidatedJson,
};

use super::model::{CreateMovement, Movement};

#[tracing::instrument(skip_all)]
pub async fn movements_index(State(pool): State<Pool>) -> impl IntoResponse {
    let mut conn = acquire(&pool).await?;
    Movement::select_all(&mut *conn).await.map(Json)
}

#[tracing::instrument(skip_all)]
pub async fn create_movement(
    State(pool): State<Pool>,
    ValidatedJson(movement): ValidatedJson<CreateMovement>,
) -> impl IntoResponse {
    let mut conn = acquire(&pool).await?;
    movement.insert_one(&mut *conn).await.map(Json)
}

#[tracing::instrument(skip_all)]
pub async fn update_movement(
    State(pool): State<Pool>,
    ValidatedJson(movement): ValidatedJson<Movement>,
) -> impl IntoResponse {
    let mut conn = acquire(&pool).await?;
    movement
        .update_one(&mut *conn)
        .await
        .map(or_404::<_, Json<_>>)
}
