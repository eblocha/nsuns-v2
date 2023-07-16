use axum::{
    routing::{delete, get},
    Router,
};

use crate::db::Pool;

use super::handler::{create_user, delete_user, update_user, users_index};

pub fn users_router() -> Router<Pool> {
    Router::new()
        .route("/", get(users_index).post(create_user).put(update_user))
        .route("/:id", delete(delete_user))
}
