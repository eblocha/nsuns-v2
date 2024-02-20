use axum::{extract::FromRef, routing::get, Router};

use crate::{db::Pool, router::State};

use super::handler::{create_max, maxes_index, update_max};

pub fn router<S: State>() -> Router<S>
where
    Pool: FromRef<S>,
{
    Router::new().route("/", get(maxes_index).post(create_max).put(update_max))
}
