use axum::{extract::State, response::IntoResponse, Json};
use axum_macros::debug_handler;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use sqlx::Transaction;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    auth::token::OwnerId,
    db::{commit_ok, Pool, DB},
    error::{extract::WithErrorRejection, OperationResult},
    maxes::model::{delete_latest_maxes, CreateMax, Max},
    reps::model::{delete_latest_reps, CreateReps, Reps},
    transaction,
};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Updates {
    pub profile_id: Uuid,
    pub movement_ids: Vec<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdatedState {
    pub maxes: Vec<Max>,
    pub reps: Vec<Reps>,
}

fn get_inc_amount(latest_reps: Option<i32>) -> f64 {
    match latest_reps {
        Some(reps) if reps >= 6 => 15_f64,
        Some(reps) if (4..=5).contains(&reps) => 10_f64,
        Some(reps) if (2..=3).contains(&reps) => 5_f64,
        _ => 0_f64,
    }
}

async fn run_updates(
    tx: &mut Transaction<'_, DB>,
    updates: Updates,
    owner_id: OwnerId,
) -> OperationResult<UpdatedState> {
    let mut new_maxes = Vec::<Max>::with_capacity(updates.movement_ids.len());
    let mut new_reps = Vec::<Reps>::with_capacity(updates.movement_ids.len());

    for movement_id in updates.movement_ids {
        let latest_max =
            Max::select_latest(movement_id, updates.profile_id, owner_id, &mut **tx).await?;

        if let Some(latest_max) = latest_max {
            let latest_reps =
                Reps::select_latest(movement_id, updates.profile_id, owner_id, &mut **tx).await?;

            let inc = get_inc_amount(latest_reps.and_then(|r| r.amount));

            let new_max = CreateMax {
                amount: latest_max.amount + inc,
                movement_id: latest_max.movement_id,
                profile_id: latest_max.profile_id,
            }
            .insert_one(owner_id, tx)
            .await?;

            let new_rep = CreateReps {
                amount: None,
                movement_id: latest_max.movement_id,
                profile_id: latest_max.profile_id,
            }
            .insert_one(owner_id, tx)
            .await?;

            new_maxes.push(new_max);
            new_reps.push(new_rep);
        }
    }

    Ok(UpdatedState {
        maxes: new_maxes,
        reps: new_reps,
    })
}

#[debug_handler]
#[tracing::instrument(skip_all)]
pub async fn updates(
    State(pool): State<Pool>,
    owner_id: OwnerId,
    WithErrorRejection(Json(updates)): WithErrorRejection<Json<Updates>>,
) -> impl IntoResponse {
    let mut tx = transaction!(&pool).await?;
    let res = run_updates(&mut tx, updates, owner_id).await.map(Json);
    commit_ok(res, tx).await
}

#[serde_as]
#[derive(Debug, Serialize, ToSchema)]
pub struct DeletedId(
    #[schema(value_type = String, format = Int64)]
    #[serde_as(as = "DisplayFromStr")]
    i64,
);

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Removed {
    pub maxes: Vec<DeletedId>,
    pub reps: Vec<DeletedId>,
}

async fn undo_updates(
    tx: &mut Transaction<'_, DB>,
    updates: Updates,
    owner_id: OwnerId,
) -> OperationResult<Removed> {
    let mut maxes = Vec::<DeletedId>::with_capacity(updates.movement_ids.len());
    let mut reps = Vec::<DeletedId>::with_capacity(updates.movement_ids.len());

    for movement_id in updates.movement_ids {
        if let Some(rep_id) =
            delete_latest_reps(updates.profile_id, movement_id, owner_id, &mut **tx).await?
        {
            reps.push(DeletedId(rep_id));
        }

        if let Some(max_id) =
            delete_latest_maxes(updates.profile_id, movement_id, owner_id, &mut **tx).await?
        {
            maxes.push(DeletedId(max_id));
        }
    }
    Ok(Removed { maxes, reps })
}

#[tracing::instrument(skip_all)]
pub async fn undo(
    State(pool): State<Pool>,
    owner_id: OwnerId,
    WithErrorRejection(Json(updates)): WithErrorRejection<Json<Updates>>,
) -> impl IntoResponse {
    let mut tx = transaction!(&pool).await?;
    let res = undo_updates(&mut tx, updates, owner_id).await.map(Json);
    commit_ok(res, tx).await
}

#[cfg(test)]
mod tests {
    use crate::updates::handler::get_inc_amount;

    #[test]
    fn test_inc_has_reps() {
        let cases = [
            (7, 15_f64),
            (6, 15_f64),
            (5, 10_f64),
            (4, 10_f64),
            (3, 5_f64),
            (2, 5_f64),
            (1, 0_f64),
            (0, 0_f64),
        ];

        for (reps, inc) in cases {
            assert_eq!(
                inc,
                get_inc_amount(Some(reps)),
                "{reps} reps did not increase max by {inc}"
            );
        }
    }

    #[test]
    fn test_inc_no_reps() {
        assert_eq!(0_f64, get_inc_amount(None));
    }
}
