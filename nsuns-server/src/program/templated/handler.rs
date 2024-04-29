use axum::{extract::State, response::IntoResponse, Json};
use transaction::commit_ok;

use crate::{
    auth::token::OwnerId,
    db::{acquire, transaction, Pool},
    error::extract::WithErrorRejection,
    response_transforms::created,
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
    let mut conn = acquire(&pool).await?;
    let mut tx = transaction(&mut *conn).await?;
    let res = validated_template
        .insert(owner_id, &mut tx)
        .await
        .map(Json)
        .map(created);

    commit_ok(res, tx).await
}
