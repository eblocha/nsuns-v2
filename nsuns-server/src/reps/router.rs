use axum::{routing::get, Router};

use crate::db::Pool;

use super::handler::{create_reps, reps_index, update_reps};

pub fn reps_router() -> Router<Pool> {
    Router::new().route("/", get(reps_index).post(create_reps).put(update_reps))
}
