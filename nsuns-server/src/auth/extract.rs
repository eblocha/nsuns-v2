use async_trait::async_trait;
use axum::extract::FromRequestParts;
use http::{request::Parts, StatusCode};

use crate::error::ErrorWithStatus;

use super::token::{Claims, OwnerId};

#[async_trait]
impl<S> FromRequestParts<S> for Claims {
    type Rejection = ErrorWithStatus<&'static str>;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Claims, Self::Rejection> {
        parts
            .extensions
            .get::<Claims>()
            .cloned()
            .ok_or(ErrorWithStatus::new(
                StatusCode::UNAUTHORIZED,
                "Unauthorized",
            ))
    }
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for OwnerId {
    type Rejection = ErrorWithStatus<&'static str>;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<OwnerId, Self::Rejection> {
        Claims::from_request_parts(parts, state)
            .await
            .map(|claims| claims.owner_id)
    }
}
