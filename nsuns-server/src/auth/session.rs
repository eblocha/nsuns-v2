use chrono::{Days, Utc};
use const_format::formatcp;
use serde::{Deserialize, Serialize};
use sqlx::{Acquire, Executor, FromRow, Transaction};
use thiserror::Error;
use time::Duration;
use tower_cookies::{
    cookie::{Expiration, SameSite},
    Cookie, Cookies,
};
use transaction::{commit_instrumented, transaction_instrumented};
use uuid::Uuid;

use crate::{
    db::{
        tracing::{
            statements::{DELETE_FROM, INSERT_INTO, SELECT, UPDATE},
            InstrumentExecutor,
        },
        transaction, DB,
    },
    db_span,
};

pub const COOKIE_NAME: &str = "SESSIONID";

#[derive(Debug, Clone)]
pub struct Session<T> {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub data: T,
    pub expiry_date: chrono::DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize, FromRow)]
struct SessionRow {
    id: Uuid,
    owner_id: Uuid,
    /// JSON-serialized data stored as a bytea
    data: Vec<u8>,
    expiry_date: chrono::DateTime<Utc>,
}

impl<T: for<'a> Deserialize<'a>> TryFrom<SessionRow> for Session<T> {
    type Error = Error;

    fn try_from(value: SessionRow) -> Result<Self, Self::Error> {
        let data = serde_json::from_slice(&value.data)?;

        Ok(Session {
            id: value.id,
            owner_id: value.owner_id,
            data,
            expiry_date: value.expiry_date,
        })
    }
}

#[derive(Debug, Clone)]
pub struct CreateSession<T> {
    /// The user which the session is for. If None, this will be an nonymous session and create a new anonymous owner.
    pub user_id: Option<Uuid>,
    /// Data associated with the session
    pub data: T,
    /// When the session will expire
    pub expiry_date: chrono::DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct UpdateSession<T> {
    pub id: Uuid,
    /// Data associated with the session
    pub data: T,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),

    #[error(transparent)]
    Serde(#[from] serde_json::Error),
}

pub async fn delete_expired(acquire: impl Acquire<'_, Database = DB>) -> Result<(), sqlx::Error> {
    let mut tx = transaction_instrumented(acquire).await?;

    // using the server's clock since that is what the expiry date is relative to
    let now = Utc::now();

    // delete expired owners
    sqlx::query("DELETE FROM owners WHERE expiry_date < $1")
        .bind(now)
        .execute((&mut *tx).instrument_executor(db_span!(DELETE_FROM, "owners")))
        .await?;

    // delete expired sessions
    sqlx::query("DELETE FROM sessions WHERE expiry_date < $1")
        .bind(now)
        .execute((&mut *tx).instrument_executor(db_span!(DELETE_FROM, "sessions")))
        .await?;

    commit_instrumented(tx).await?;

    Ok(())
}

pub async fn get_user_owner_id(
    executor: impl Executor<'_, Database = DB>,
    user_id: Uuid,
) -> Result<Uuid, Error> {
    Ok(
        sqlx::query_as::<_, (Uuid,)>(formatcp!("{SELECT} owner_id FROM users WHERE id = $1"))
            .bind(user_id)
            .fetch_one(executor.instrument_executor(db_span!(SELECT, "users")))
            .await?
            .0,
    )
}

pub async fn new_user_session(
    tx: &mut Transaction<'_, DB>,
    user_id: Option<Uuid>,
    cookies: Cookies,
) -> Result<Session<()>, Error> {
    let expiry_date = Utc::now()
        .checked_add_days(Days::new(2))
        .expect("future timestamp does not overflow");

    // create a new session
    let session = create_session(
        tx,
        CreateSession {
            data: (),
            expiry_date,
            user_id,
        },
    )
    .await?;

    // add the session id to the response cookies
    let cookie = Cookie::build(COOKIE_NAME, session.id.to_string())
        .path("/")
        .expires(Expiration::Session)
        .max_age(Duration::days(2))
        .same_site(SameSite::Strict)
        .secure(true)
        .http_only(true)
        .finish();

    cookies.add(cookie);

    Ok(session)
}

pub async fn create_session<T: Serialize>(
    tx: &mut Transaction<'_, DB>,
    record: CreateSession<T>,
) -> Result<Session<T>, Error> {
    let data = serde_json::to_string(&record.data)?;

    let owner_id = if let Some(user_id) = record.user_id {
        // look up the user's owner_id to store with the session
        get_user_owner_id(&mut **tx, user_id).await?
    } else {
        // create a new owner_id for this session containing the same expiry date
        sqlx::query_as::<_, (Uuid,)>(formatcp!(
            "{INSERT_INTO} owners (expiry_date) VALUES ($1) RETURNING id"
        ))
        .bind(record.expiry_date)
        .fetch_one((&mut **tx).instrument_executor(db_span!(INSERT_INTO, "owners")))
        .await?
        .0
    };

    // store the session pointing to the owner

    let session_id = sqlx::query_as::<_, (Uuid,)>(formatcp!(
        "{INSERT_INTO} sessions (owner_id, data, expiry_date) VALUES ($1, $2, $3) RETURNING id"
    ))
    .bind(owner_id)
    .bind(data.as_bytes())
    .bind(record.expiry_date)
    .fetch_one((&mut **tx).instrument_executor(db_span!(INSERT_INTO, "sessions")))
    .await?
    .0;

    Ok(Session {
        id: session_id,
        owner_id,
        data: record.data,
        expiry_date: record.expiry_date,
    })
}

pub async fn get_session<T: for<'a> Deserialize<'a>>(
    executor: impl Executor<'_, Database = DB>,
    id: Uuid,
) -> Result<Option<Session<T>>, Error> {
    let row = sqlx::query_as::<_, SessionRow>("SELECT * FROM sessions WHERE id = $1")
        .bind(id)
        .fetch_optional(executor.instrument_executor(db_span!(SELECT, "sessions")))
        .await?;

    if let Some(row) = row {
        row.try_into().map(Some)
    } else {
        Ok(None)
    }
}

pub async fn delete_sessions_for_user(
    tx: &mut Transaction<'_, DB>,
    user_id: Uuid,
) -> Result<(), Error> {
    let owner_id = get_user_owner_id(&mut **tx, user_id).await?;

    sqlx::query("DELETE FROM sessions WHERE owner_id = $1")
        .bind(owner_id)
        .execute((&mut **tx).instrument_executor(db_span!(DELETE_FROM, "sessions")))
        .await?;

    Ok(())
}

pub async fn update_session<T: Serialize>(
    executor: impl Executor<'_, Database = DB>,
    session: &UpdateSession<T>,
) -> Result<(), Error> {
    let data = serde_json::to_string(&session.data)?;

    sqlx::query("UPDATE sessions SET data = $1 WHERE id = $2")
        .bind(data.as_bytes())
        .bind(session.id)
        .execute(executor.instrument_executor(db_span!(UPDATE, "sessions")))
        .await?;

    Ok(())
}

pub async fn delete_session(
    executor: impl Executor<'_, Database = DB>,
    id: Uuid,
) -> Result<(), Error> {
    sqlx::query("DELETE FROM sessions WHERE id = $1")
        .bind(id)
        .execute(executor.instrument_executor(db_span!(DELETE_FROM, "sessions")))
        .await?;

    Ok(())
}

pub fn get_session_id_from_cookies(cookies: &Cookies) -> Option<Uuid> {
    cookies
        .get(COOKIE_NAME)
        .and_then(|cookie| cookie.value().parse().ok())
}
