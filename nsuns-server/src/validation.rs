use async_trait::async_trait;
use axum::{
    extract::{rejection::JsonRejection, FromRequest, Request},
    Json,
};
use serde::{de, Deserialize};
use validator::{Validate, ValidationErrors};

use crate::error::ErrorWithStatus;

/// A validated `T`
///
/// It is impossible to construct this without first validating `T`.
#[derive(Debug, Clone, Copy, Default)]
pub struct Validated<T>(T);

impl<T> Validated<T> {
    pub fn from_validate(value: T) -> Result<Self, ValidationErrors>
    where
        T: Validate,
    {
        value.validate()?;
        Ok(Validated(value))
    }

    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> AsRef<T> for Validated<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<'de, T: Validate + Deserialize<'de>> Deserialize<'de> for Validated<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = T::deserialize(deserializer)?;

        Validated::from_validate(value).map_err(|validation_err| {
            de::Error::custom(format!("Input validation error: [{validation_err}]"))
        })
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    Json<Validated<T>>: FromRequest<S, Rejection = JsonRejection>,
    S: Send + Sync,
{
    type Rejection = ErrorWithStatus<String>;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(Validated(value)) = Json::from_request(req, state)
            .await
            .map_err(|json_err| ErrorWithStatus::new(json_err.status(), json_err.body_text()))?;

        Ok(ValidatedJson(value))
    }
}
