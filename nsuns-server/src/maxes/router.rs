use axum::{routing::get, Router};

use crate::db::Pool;

use super::handler::{create_max, maxes_index};

pub fn maxes_router() -> Router<Pool> {
    Router::new().route("/", get(maxes_index).post(create_max))
}
