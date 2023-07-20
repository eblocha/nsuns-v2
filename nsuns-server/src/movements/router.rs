use axum::{routing::get, Router};

use crate::db::Pool;

use super::handler::{create_movement, movements_index};

pub fn movements_router() -> Router<Pool> {
    Router::new().route("/", get(movements_index).post(create_movement))
}
