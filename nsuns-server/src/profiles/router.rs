use axum::{extract::FromRef, routing::get, Router};

use crate::{db::Pool, router::State};

use super::handler::{create_profile, delete_profile, get_profile, profiles_index, update_profile};

pub fn router<S: State>() -> Router<S>
where
    Pool: FromRef<S>,
{
    Router::new()
        .route(
            "/",
            get(profiles_index).post(create_profile).put(update_profile),
        )
        .route("/:id", get(get_profile).delete(delete_profile))
}
