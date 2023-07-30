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
    error::LogError,
    util::{created, or_404},
    validation::ValidatedJson,
};

use super::model::{CreateReps, Reps, UpdateReps};

#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
#[serde(rename_all = "camelCase")]
pub struct RepsQuery {
    pub profile_id: Uuid,
}

pub async fn reps_index(
    State(pool): State<Pool>,
    Query(query): Query<RepsQuery>,
) -> impl IntoResponse {
    Reps::select_for_profile(query.profile_id, &pool)
        .await
        .map(Json)
        .log_error()
}

pub async fn create_reps(
    State(pool): State<Pool>,
    ValidatedJson(reps): ValidatedJson<CreateReps>,
) -> impl IntoResponse {
    reps.insert_one(&pool)
        .await
        .map(Json)
        .map(created)
        .log_error()
}

pub async fn update_reps(
    State(pool): State<Pool>,
    ValidatedJson(reps): ValidatedJson<UpdateReps>,
) -> impl IntoResponse {
    reps.update_one(&pool)
        .await
        .map(or_404::<_, Json<_>>)
        .log_error()
}
