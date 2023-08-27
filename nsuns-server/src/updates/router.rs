use axum::{routing::post, Router};

use crate::db::Pool;

use super::handler::{undo, updates};

pub fn router() -> Router<Pool> {
    Router::new().route("/", post(updates).delete(undo))
}
