use axum::{
    extract::FromRef,
    routing::{delete, post},
    Router,
};

use crate::{db::Pool, router::State};

use super::handler::{create_set, delete_set, update_set};

pub fn router<S: State>() -> Router<S>
where
    Pool: FromRef<S>,
{
    Router::new()
        .route("/", post(create_set).put(update_set))
        .route("/:id", delete(delete_set))
}
