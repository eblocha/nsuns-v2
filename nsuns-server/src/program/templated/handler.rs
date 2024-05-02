use axum::{extract::State, response::IntoResponse, Json};

use crate::{
    auth::token::OwnerId,
    db::{commit_ok, Pool},
    error::extract::WithErrorRejection,
    response_transforms::created,
    transaction,
    validation::Validated,
};

use super::model::TemplatedProgram;

#[tracing::instrument(skip_all)]
pub async fn create_from_template(
    State(pool): State<Pool>,
    owner_id: OwnerId,
    WithErrorRejection(Json(validated_template)): WithErrorRejection<
        Json<Validated<TemplatedProgram>>,
    >,
) -> impl IntoResponse {
    let mut tx = transaction!(&pool).await?;
    let res = validated_template
        .insert(owner_id, &mut tx)
        .await
        .map(Json)
        .map(created);

    commit_ok(res, tx).await
}
