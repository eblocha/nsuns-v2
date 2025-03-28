use anyhow::{anyhow, Context};
use chrono::{serde::ts_milliseconds, DateTime, Days, Utc};
use const_format::formatcp;
use http::StatusCode;
use jsonwebtoken::{
    decode, encode,
    errors::{Error, ErrorKind},
    DecodingKey, EncodingKey, Header, Validation,
};
use secrecy::ExposeSecret;
use serde::{Deserialize, Serialize};
use sqlx::{Executor, FromRow};
use time::Duration;
use tower_cookies::{
    cookie::{CookieBuilder, SameSite},
    Cookie, Cookies,
};
use uuid::Uuid;

use crate::{
    db::{
        tracing::{
            statements::{DELETE_FROM, INSERT_INTO, SELECT},
            InstrumentExecutor,
        },
        DB,
    },
    db_span,
    error::{ErrorWithStatus, OperationResult},
    into_log_server_error,
};

use super::settings::AuthSettings;

const TABLE: &str = "sessions";

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Claims {
    pub id: Uuid,
    pub owner_id: OwnerId,
    pub user_id: Option<Uuid>,
    #[serde(with = "ts_milliseconds")]
    pub exp: DateTime<Utc>,
}

impl Claims {
    /// Create a new session
    ///
    /// Pass [`None`] for `user_id` for anonymous users.
    /// Pass [`None`] for `exp` to generate a new expiry date.
    #[must_use]
    pub async fn insert_one(
        executor: impl Executor<'_, Database = DB>,
        owner_id: Uuid,
        user_id: Option<Uuid>,
        exp: Option<DateTime<Utc>>,
    ) -> OperationResult<Claims> {
        let exp = exp.unwrap_or_else(create_new_expiry_date);
        sqlx::query_as::<_, Claims>(formatcp!(
            "{INSERT_INTO} {TABLE} (user_id, owner_id, exp) VALUES ($1, $2, $3) RETURNING *"
        ))
        .bind(user_id)
        .bind(owner_id)
        .bind(exp)
        .fetch_one(executor.instrument_executor(db_span!(INSERT_INTO, TABLE)))
        .await
        .context("failed to create a session")
        .map_err(into_log_server_error!())
    }

    #[must_use]
    pub async fn select_one(
        id: Uuid,
        executor: impl Executor<'_, Database = DB>,
    ) -> OperationResult<Option<Claims>> {
        sqlx::query_as::<_, Claims>(formatcp!("{SELECT} * FROM {TABLE} WHERE id = $1"))
            .bind(id)
            .fetch_optional(executor.instrument_executor(db_span!(SELECT, TABLE)))
            .await
            .context("failed to select session")
            .map_err(into_log_server_error!())
    }

    /// Delete a session
    ///
    /// This returns [`None`] if the session did not exist, or [`Some`] if it did.
    #[must_use]
    pub async fn revoke(
        &self,
        executor: impl Executor<'_, Database = DB>,
    ) -> OperationResult<Option<()>> {
        sqlx::query(formatcp!("{DELETE_FROM} {TABLE} WHERE id = $1"))
            .bind(self.id)
            .execute(executor.instrument_executor(db_span!(DELETE_FROM, TABLE)))
            .await
            .with_context(|| format!("failed to revoke token with id={}", self.id))
            .map_err(into_log_server_error!())
            .map(|res| {
                if res.rows_affected() > 0 {
                    Some(())
                } else {
                    None
                }
            })
    }
}

/// Decode claims from cookies, returning [`None`] if the cookie does not contain a token or contains an invalid token.
pub fn decode_claims_from_cookies(keys: &JwtKeys, cookies: &Cookies) -> Option<Claims> {
    cookies
        .get(COOKIE_NAME)
        .and_then(|cookie| keys.decode(cookie.value()).ok())
}

/// The authenticated resource owner id
#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct OwnerId(Uuid);

impl OwnerId {
    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

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
    CookieBuilder::new(COOKIE_NAME, token)
        .path("/")
        .max_age(Duration::days(2))
        .http_only(true)
        .same_site(SameSite::Lax)
        .build()
}

/// Create an empty cookie suitable for instructing the client to remove their JWT cookie
#[must_use]
pub fn create_empty_cookie<'c>() -> Cookie<'c> {
    CookieBuilder::new(COOKIE_NAME, "")
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .build()
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
