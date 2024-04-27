use axum::{
    extract::FromRef,
    routing::{get, post},
    Router,
};

use crate::{db::Pool, router::State};

use super::{
    handler::{agent_info, anonymous, login, logout},
    token::JwtKeys,
};

pub fn router<S: State>() -> Router<S>
where
    Pool: FromRef<S>,
    JwtKeys: FromRef<S>,
{
    Router::new()
        .route("/login", post(login))
        .route("/anonymous", post(anonymous))
        .route("/logout", post(logout))
        .route("/user-info", get(agent_info))
}
