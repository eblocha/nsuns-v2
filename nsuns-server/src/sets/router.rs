use axum::{
    routing::{delete, post},
    Router,
};

use crate::db::Pool;

use super::handler::{create_set, delete_set, update_set};

pub fn router() -> Router<Pool> {
    Router::new()
        .route("/", post(create_set).put(update_set))
        .route("/:id", delete(delete_set))
}
