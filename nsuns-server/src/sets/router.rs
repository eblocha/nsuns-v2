use axum::{
    routing::{delete, post},
    Router,
};

use crate::db::Pool;

use super::handler::{create_set, delete_set};

pub fn sets_router() -> Router<Pool> {
    Router::new()
        .route("/", post(create_set))
        .route("/:id", delete(delete_set))
}