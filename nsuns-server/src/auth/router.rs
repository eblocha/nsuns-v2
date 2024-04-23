use axum::{extract::FromRef, routing::post, Router};

use crate::{db::Pool, router::State};

use super::handler::{anonymous_session, login, logout};

pub fn router<S: State>() -> Router<S>
where
    Pool: FromRef<S>,
{
    Router::new()
        .route("/login", post(login))
        .route("/anonymous", post(anonymous_session))
        .route("/logout", post(logout))
}
