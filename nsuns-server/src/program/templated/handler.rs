use axum::{extract::State, response::IntoResponse, Json};
use transaction::commit_ok;

use crate::{
    auth::token::OwnerId,
    db::{acquire, transaction, Pool},
    response_transforms::created,
    validation::ValidatedJson,
};

use super::model::TemplatedProgram;

#[tracing::instrument(skip_all)]
pub async fn create_from_template(
    State(pool): State<Pool>,
    owner_id: OwnerId,
    ValidatedJson(template): ValidatedJson<TemplatedProgram>,
) -> impl IntoResponse {
    let mut conn = acquire(&pool).await?;
    let mut tx = transaction(&mut *conn).await?;
    let res = template
        .insert(owner_id, &mut tx)
        .await
        .map(Json)
        .map(created);

    commit_ok(res, tx).await
}
