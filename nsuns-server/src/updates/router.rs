use axum::{routing::post, Router};

use crate::db::Pool;

use super::hander::updates;

pub fn updates_router() -> Router<Pool> {
    Router::new().route("/", post(updates))
}
