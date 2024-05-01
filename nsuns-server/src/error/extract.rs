use async_trait::async_trait;
use axum::{
    extract::{
        rejection::{JsonRejection, PathRejection, QueryRejection, TypedHeaderRejection},
        FromRequest, FromRequestParts, Path, Query,
    },
    Json, TypedHeader,
};
use http::{request::Parts, Request, StatusCode};
use tower_cookies::Cookies;

use super::ErrorWithStatus;

/// Wraps an extractor to return an `ErrorWithStatus` rejection instead,
/// which will be converted to a JSON response.
pub struct WithErrorRejection<T>(pub T);

#[async_trait]
impl<T, S, B> FromRequest<S, B> for WithErrorRejection<Json<T>>
where
    Json<T>: FromRequest<S, B, Rejection = JsonRejection>,
    S: Send + Sync,
    B: Send + 'static,
{
    type Rejection = ErrorWithStatus<String>;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let value = Json::from_request(req, state)
            .await
            .map_err(|e| ErrorWithStatus::new(e.status(), e.body_text()))?;

        Ok(WithErrorRejection(value))
    }
}

#[async_trait]
impl<T, S> FromRequestParts<S> for WithErrorRejection<Query<T>>
where
    Query<T>: FromRequestParts<S, Rejection = QueryRejection>,
    S: Send + Sync,
{
    type Rejection = ErrorWithStatus<String>;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let value = Query::from_request_parts(parts, state)
            .await
            .map_err(|e| ErrorWithStatus::new(e.status(), e.body_text()))?;

        Ok(WithErrorRejection(value))
    }
}

#[async_trait]
impl<T, S> FromRequestParts<S> for WithErrorRejection<TypedHeader<T>>
where
    TypedHeader<T>: FromRequestParts<S, Rejection = TypedHeaderRejection>,
    S: Send + Sync,
{
    type Rejection = ErrorWithStatus<String>;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let value = TypedHeader::from_request_parts(parts, state)
            .await
            .map_err(|e| ErrorWithStatus::new(StatusCode::BAD_REQUEST, e.to_string()))?;

        Ok(WithErrorRejection(value))
    }
}

#[async_trait]
impl<T, S> FromRequestParts<S> for WithErrorRejection<Path<T>>
where
    Path<T>: FromRequestParts<S, Rejection = PathRejection>,
    S: Send + Sync,
{
    type Rejection = ErrorWithStatus<String>;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let value = Path::from_request_parts(parts, state)
            .await
            .map_err(|e: PathRejection| ErrorWithStatus::new(e.status(), e.body_text()))?;

        Ok(WithErrorRejection(value))
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for WithErrorRejection<Cookies>
where
    Cookies: FromRequestParts<S, Rejection = (StatusCode, &'static str)>,
    S: Send + Sync,
{
    type Rejection = ErrorWithStatus<&'static str>;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let value = Cookies::from_request_parts(parts, state)
            .await
            .map_err(|e| ErrorWithStatus::new(e.0, e.1))?;

        Ok(WithErrorRejection(value))
    }
}
