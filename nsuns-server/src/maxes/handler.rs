use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{db::Pool, error::LogError, util::created};

use super::model::{CreateMax, Max};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaxesQuery {
    pub profile_id: Uuid,
}

pub async fn maxes_index(
    State(pool): State<Pool>,
    Query(query): Query<MaxesQuery>,
) -> impl IntoResponse {
    Max::select_for_profile(query.profile_id, &pool)
        .await
        .map(Json)
        .log_error()
}

pub async fn create_max(State(pool): State<Pool>, Json(max): Json<CreateMax>) -> impl IntoResponse {
    max.insert_one(&pool)
        .await
        .map(Json)
        .map(created)
        .log_error()
}
