use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use utoipa::IntoParams;
use uuid::Uuid;

use crate::{
    db::{transaction::acquire, Pool},
    response_transforms::{created, or_404},
    validation::ValidatedJson,
};

use super::model::{CreateReps, Reps, UpdateReps};

#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
#[serde(rename_all = "camelCase")]
pub struct RepsQuery {
    pub profile_id: Uuid,
}

#[tracing::instrument(skip_all)]
pub async fn reps_index(
    State(pool): State<Pool>,
    Query(query): Query<RepsQuery>,
) -> impl IntoResponse {
    let mut conn = acquire(&pool).await?;
    Reps::select_for_profile(query.profile_id, &mut *conn)
        .await
        .map(Json)
}

#[tracing::instrument(skip_all)]
pub async fn create_reps(
    State(pool): State<Pool>,
    ValidatedJson(reps): ValidatedJson<CreateReps>,
) -> impl IntoResponse {
    let mut conn = acquire(&pool).await?;
    reps.insert_one(&mut *conn).await.map(Json).map(created)
}

#[tracing::instrument(skip_all)]
pub async fn update_reps(
    State(pool): State<Pool>,
    ValidatedJson(reps): ValidatedJson<UpdateReps>,
) -> impl IntoResponse {
    let mut conn = acquire(&pool).await?;
    reps.update_one(&mut *conn).await.map(or_404::<_, Json<_>>)
}
