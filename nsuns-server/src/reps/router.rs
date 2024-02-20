use axum::{extract::FromRef, routing::get, Router};

use crate::{db::Pool, router::State};

use super::handler::{create_reps, reps_index, update_reps};

pub fn router<S: State>() -> Router<S>
where
    Pool: FromRef<S>,
{
    Router::new().route("/", get(reps_index).post(create_reps).put(update_reps))
}
