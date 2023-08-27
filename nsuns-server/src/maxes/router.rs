use axum::{routing::get, Router};

use crate::db::Pool;

use super::handler::{create_max, maxes_index, update_max};

pub fn router() -> Router<Pool> {
    Router::new().route("/", get(maxes_index).post(create_max).put(update_max))
}
