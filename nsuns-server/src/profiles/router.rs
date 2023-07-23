use axum::{routing::get, Router};

use crate::db::Pool;

use super::handler::{create_profile, delete_profile, get_profile, profiles_index, update_profile};

pub fn profiles_router() -> Router<Pool> {
    Router::new()
        .route(
            "/",
            get(profiles_index).post(create_profile).put(update_profile),
        )
        .route("/:id", get(get_profile).delete(delete_profile))
}
