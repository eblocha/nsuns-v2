use std::sync::Arc;

use anyhow::anyhow;
use async_trait::async_trait;
use axum::{
    extract::FromRequestParts,
    response::{IntoResponse, Response},
};
use http::request::Parts;

use crate::error::ErrorWithStatus;

use super::{
    middleware::ClaimsResult,
    token::{Claims, OwnerId},
};

pub type ClaimsRejection = Response;

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for ClaimsResult {
    type Rejection = ClaimsRejection;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<ClaimsResult>()
            .cloned()
            .ok_or_else(|| Arc::new(ErrorWithStatus::from(
                anyhow!("No claims found in request extensions. Is the route behind the token middleware?")
            ))).map_err(|e| e.as_ref().into_response())
    }
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Claims {
    type Rejection = ClaimsRejection;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        ClaimsResult::from_request_parts(parts, state)
            .await?
            .0
            .map_err(|e| e.as_ref().into_response())
    }
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for OwnerId {
    type Rejection = ClaimsRejection;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<OwnerId, Self::Rejection> {
        Claims::from_request_parts(parts, state)
            .await
            .map(|claims| claims.owner_id)
    }
}
