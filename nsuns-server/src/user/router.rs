use axum::{routing::get, Router};

use crate::db::Pool;

use super::handler::{create_user, delete_user, get_user, is_taken, update_user, users_index};

pub fn users_router() -> Router<Pool> {
    Router::new()
        .route("/", get(users_index).post(create_user).put(update_user))
        .route("/:id", get(get_user).delete(delete_user))
        .route("/validation/is-taken/:username", get(is_taken))
}
