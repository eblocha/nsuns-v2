use async_trait::async_trait;
use axum::extract::FromRequestParts;
use http::{request::Parts, StatusCode};

use super::session::Session;

pub type AuthSession = Session<()>;

#[async_trait]
impl<S> FromRequestParts<S> for AuthSession {
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> Result<AuthSession, Self::Rejection> {
        parts
            .extensions
            .get::<AuthSession>()
            .cloned()
            .ok_or(StatusCode::UNAUTHORIZED)
    }
}
