use async_trait::async_trait;
use axum::{
    extract::{rejection::JsonRejection, FromRequest},
    http::{Request, StatusCode},
    Json,
};
use validator::Validate;

use crate::error::ErrorWithStatus;

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<T, S, B> FromRequest<S, B> for ValidatedJson<T>
where
    T: Validate,
    Json<T>: FromRequest<S, B, Rejection = JsonRejection>,
    S: Send + Sync,
    B: Send + 'static,
{
    type Rejection = ErrorWithStatus<String>;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::from_request(req, state)
            .await
            .map_err(|json_err| ErrorWithStatus::new(json_err.status(), json_err.body_text()))?;
        value.validate().map_err(|validation_err| {
            let message = format!("Input validation error: [{validation_err}]").replace('\n', ", ");
            ErrorWithStatus::new(StatusCode::UNPROCESSABLE_ENTITY, message)
        })?;
        Ok(ValidatedJson(value))
    }
}
