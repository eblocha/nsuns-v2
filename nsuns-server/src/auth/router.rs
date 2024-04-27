use axum::{
    extract::FromRef,
    routing::{get, post},
    Router,
};

use crate::{db::Pool, router::State};

use super::{
    handler::{anonymous, login, logout, user_info},
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
        .route("/user-info", get(user_info))
}
