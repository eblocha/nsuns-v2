use async_trait::async_trait;
use axum::extract::FromRequestParts;
use http::{request::Parts, StatusCode};

use super::token::Claims;

#[async_trait]
impl<S> FromRequestParts<S> for Claims {
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Claims, Self::Rejection> {
        parts
            .extensions
            .get::<Claims>()
            .cloned()
            .ok_or(StatusCode::UNAUTHORIZED)
    }
}
