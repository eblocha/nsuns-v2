use axum::{extract::State, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use sqlx::Transaction;
use uuid::Uuid;

use crate::{
    db::{commit_ok, transaction, Pool, DB},
    error::{IntoResult, LogError, Result},
    maxes::model::{CreateMax, Max},
    reps::model::{CreateReps, Reps},
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Updates {
    pub profile_id: Uuid,
    pub movement_ids: Vec<i32>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatedState {
    pub maxes: Vec<Max>,
    pub reps: Vec<Reps>,
}

async fn run_updates(tx: &mut Transaction<'_, DB>, updates: Updates) -> Result<UpdatedState> {
    let mut new_maxes = Vec::<Max>::with_capacity(updates.movement_ids.len());
    let mut new_reps = Vec::<Reps>::with_capacity(updates.movement_ids.len());

    for movement_id in updates.movement_ids {
        let latest_max = Max::select_latest(movement_id, updates.profile_id, &mut **tx).await?;

        if let Some(latest_max) = latest_max {
            let latest_reps =
                Reps::select_latest(movement_id, updates.profile_id, &mut **tx).await?;

            let inc = match latest_reps {
                Some(reps) => match reps.amount {
                    Some(amt) if amt >= 6 => 15_f64,
                    Some(amt) if amt >= 4 && amt <= 5 => 10_f64,
                    Some(amt) if amt >= 2 && amt <= 3 => 5_f64,
                    _ => 0_f64,
                },
                _ => 0_f64,
            };

            let new_max = CreateMax {
                amount: latest_max.amount + inc,
                movement_id: latest_max.movement_id,
                profile_id: latest_max.profile_id,
            }
            .insert_one(&mut **tx)
            .await?;

            let new_rep = CreateReps {
                amount: None,
                movement_id: latest_max.movement_id,
                profile_id: latest_max.profile_id,
            }
            .insert_one(&mut **tx)
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

pub async fn updates(State(pool): State<Pool>, Json(updates): Json<Updates>) -> impl IntoResponse {
    let mut tx = transaction(&pool).await.log_error()?;
    let res = run_updates(&mut tx, updates).await.map(Json).into_result();
    commit_ok(res, tx).await.log_error()
}
