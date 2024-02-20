use axum::{extract::FromRef, routing::post, Router};

use crate::{db::Pool, router::State};

use super::handler::{undo, updates};

pub fn router<S: State>() -> Router<S>
where
    Pool: FromRef<S>,
{
    Router::new().route("/", post(updates).delete(undo))
}
