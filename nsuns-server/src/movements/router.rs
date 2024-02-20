use axum::{extract::FromRef, routing::get, Router};

use crate::{db::Pool, router::State};

use super::handler::{create_movement, movements_index, update_movement};

pub fn router<S: State>() -> Router<S>
where
    Pool: FromRef<S>,
{
    Router::new().route(
        "/",
        get(movements_index)
            .post(create_movement)
            .put(update_movement),
    )
}
