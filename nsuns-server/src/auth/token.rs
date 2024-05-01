use anyhow::anyhow;
use chrono::{serde::ts_milliseconds, DateTime, Days, Utc};
use http::StatusCode;
use jsonwebtoken::{
    decode, encode,
    errors::{Error, ErrorKind},
    DecodingKey, EncodingKey, Header, Validation,
};
use secrecy::ExposeSecret;
use serde::{Deserialize, Serialize};
use time::Duration;
use tower_cookies::{cookie::SameSite, Cookie};
use uuid::Uuid;

use crate::error::ErrorWithStatus;

use super::settings::AuthSettings;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub owner_id: OwnerId,
    pub user_id: Option<Uuid>,
    #[serde(with = "ts_milliseconds")]
    pub exp: DateTime<Utc>,
}

impl Claims {
    #[must_use]
    pub fn generate(owner_id: Uuid, user_id: Option<Uuid>) -> Self {
        Self {
            owner_id: OwnerId(owner_id),
            user_id,
            exp: create_new_expiry_date(),
        }
    }
}

/// The authenticated resource owner id
#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct OwnerId(Uuid);

pub const COOKIE_NAME: &str = "JWT";

#[derive(Clone)]
pub struct JwtKeys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl JwtKeys {
    pub fn decode(&self, token: &str) -> Result<Claims, Error> {
        let token_data = decode::<Claims>(token, &self.decoding, &Validation::default())?;

        Ok(token_data.claims)
    }
    pub fn encode(&self, claims: &Claims) -> Result<String, Error> {
        encode(&Header::default(), claims, &self.encoding)
    }
}

impl<'a> From<&'a AuthSettings> for JwtKeys {
    fn from(value: &'a AuthSettings) -> Self {
        Self {
            encoding: EncodingKey::from_secret(
                value.jwt_encoding_secret.expose_secret().as_bytes(),
            ),
            decoding: DecodingKey::from_secret(
                value.jwt_decoding_secret.expose_secret().as_bytes(),
            ),
        }
    }
}

impl From<Error> for ErrorWithStatus<anyhow::Error> {
    fn from(error: Error) -> Self {
        let status_code = match error.kind() {
            ErrorKind::InvalidToken => StatusCode::BAD_REQUEST,
            ErrorKind::MissingRequiredClaim(_)
            | ErrorKind::ExpiredSignature
            | ErrorKind::InvalidIssuer
            | ErrorKind::InvalidAudience
            | ErrorKind::InvalidSubject
            | ErrorKind::ImmatureSignature
            | ErrorKind::InvalidAlgorithm
            | ErrorKind::MissingAlgorithm => StatusCode::UNAUTHORIZED,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let anyhow_err = if status_code.is_server_error() {
            error.into()
        } else {
            anyhow!("")
        };

        ErrorWithStatus::new(status_code, anyhow_err)
    }
}

/// Create a cookie for the JWT
#[must_use]
pub fn create_token_cookie<'c>(token: String) -> Cookie<'c> {
    Cookie::build(COOKIE_NAME, token)
        .path("/")
        .max_age(Duration::days(2))
        .http_only(true)
        .same_site(SameSite::Lax)
        .finish()
}

/// Create an empty cookie suitable for instructing the client to remove their JWT cookie
#[must_use]
pub fn create_empty_cookie<'c>() -> Cookie<'c> {
    Cookie::build(COOKIE_NAME, "")
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .finish()
}

/// Create an expiry date for a new JWT
///
/// # Panics
/// Panics if the new date would overflow an i64 in its seconds representation
#[must_use]
pub fn create_new_expiry_date() -> DateTime<Utc> {
    Utc::now()
        .checked_add_days(Days::new(2))
        .expect("future timestamp does not overflow")
}
