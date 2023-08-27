use axum::{routing::get, Router};

use crate::db::Pool;

use super::handler::{create_movement, movements_index, update_movement};

pub fn router() -> Router<Pool> {
    Router::new().route(
        "/",
        get(movements_index)
            .post(create_movement)
            .put(update_movement),
    )
}
