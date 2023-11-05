use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use utoipa::IntoParams;
use uuid::Uuid;

use crate::{
    db::Pool,
    response_transforms::{created, or_404},
    validation::ValidatedJson,
};

use super::model::{CreateMax, Max, UpdateMax};

#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
#[serde(rename_all = "camelCase")]
pub struct MaxesQuery {
    pub profile_id: Uuid,
}

#[tracing::instrument(skip_all)]
pub async fn maxes_index(
    State(pool): State<Pool>,
    Query(query): Query<MaxesQuery>,
) -> impl IntoResponse {
    Max::select_for_profile(query.profile_id, &pool)
        .await
        .map(Json)
}

#[tracing::instrument(skip_all)]
pub async fn create_max(
    State(pool): State<Pool>,
    ValidatedJson(max): ValidatedJson<CreateMax>,
) -> impl IntoResponse {
    max.insert_one(&pool).await.map(Json).map(created)
}

#[tracing::instrument(skip_all)]
pub async fn update_max(
    State(pool): State<Pool>,
    ValidatedJson(max): ValidatedJson<UpdateMax>,
) -> impl IntoResponse {
    max.update_one(&pool).await.map(or_404::<_, Json<_>>)
}
