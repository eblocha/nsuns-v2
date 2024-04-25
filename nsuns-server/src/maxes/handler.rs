use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use utoipa::IntoParams;
use uuid::Uuid;

use crate::{
    auth::token::OwnerId,
    db::{acquire, Pool},
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
    owner_id: OwnerId,
) -> impl IntoResponse {
    let mut conn = acquire(&pool).await?;

    Max::select_for_profile(query.profile_id, owner_id, &mut *conn)
        .await
        .map(Json)
}

#[tracing::instrument(skip_all)]
pub async fn create_max(
    State(pool): State<Pool>,
    owner_id: OwnerId,
    ValidatedJson(max): ValidatedJson<CreateMax>,
) -> impl IntoResponse {
    let mut conn = acquire(&pool).await?;
    max.insert_one(owner_id, &mut *conn)
        .await
        .map(Json)
        .map(created)
}

#[tracing::instrument(skip_all)]
pub async fn update_max(
    State(pool): State<Pool>,
    owner_id: OwnerId,
    ValidatedJson(max): ValidatedJson<UpdateMax>,
) -> impl IntoResponse {
    let mut conn = acquire(&pool).await?;
    max.update_one(owner_id, &mut *conn)
        .await
        .map(or_404::<_, Json<_>>)
}
