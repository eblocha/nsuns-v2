use anyhow::{anyhow, Context};
use axum::{
    extract::{Query, State},
    headers::{authorization::Basic, Authorization},
    response::Redirect,
    TypedHeader,
};
use http::StatusCode;
use serde::Deserialize;
use sqlx::Transaction;
use tower_cookies::{Cookie, Cookies};
use transaction::{acquire, commit_ok};

use crate::{
    db::{transaction, Pool, DB},
    error::{ErrorWithStatus, OperationResult},
    into_log_server_error,
};

use super::{
    session::{
        delete_session, delete_sessions_for_user, get_session_id_from_cookies, new_user_session,
        COOKIE_NAME,
    },
    user::authenticate,
};

#[derive(Debug, Deserialize)]
pub struct LoginQuery {
    pub next: Option<String>,
}

impl LoginQuery {
    pub fn to_redirect(&self) -> Redirect {
        if let Some(ref next) = self.next {
            // TODO handle invalid URI
            Redirect::to(next.as_str())
        } else {
            Redirect::to("/")
        }
    }
}

async fn login_user(
    tx: &mut Transaction<'_, DB>,
    auth: Basic,
    cookies: Cookies,
) -> OperationResult<()> {
    // authenticate user
    let user = match authenticate(&mut **tx, auth)
        .await
        .context("Failed to authenticate")
        .map_err(into_log_server_error!())?
    {
        Some(user) => user,
        None => {
            return Err(ErrorWithStatus::new(
                StatusCode::UNAUTHORIZED,
                anyhow!("Bad credentials"),
            ))
        }
    };

    // delete current user session if it exists
    delete_sessions_for_user(tx, user.id)
        .await
        .with_context(|| format!("Failed to delete sessions for username={}", user.username))
        .map_err(into_log_server_error!())?;

    // delete session stored in cookie if it exists
    if let Some(session_id) = get_session_id_from_cookies(&cookies) {
        delete_session(&mut **tx, session_id)
            .await
            .context("failed to delete current session")
            .map_err(into_log_server_error!())?;
    }

    // create a new session
    new_user_session(tx, Some(user.id), cookies)
        .await
        .context("failed to create new session")
        .map_err(into_log_server_error!())?;

    Ok(())
}

pub async fn login(
    State(pool): State<Pool>,
    TypedHeader(Authorization(creds)): TypedHeader<Authorization<Basic>>,
    Query(query): Query<LoginQuery>,
    cookies: Cookies,
) -> OperationResult<Redirect> {
    let mut tx = transaction(&pool).await?;

    let res = login_user(&mut tx, creds, cookies).await;

    commit_ok(res, tx).await?;

    Ok(query.to_redirect())
}

pub async fn anonymous_session(
    State(pool): State<Pool>,
    Query(query): Query<LoginQuery>,
    cookies: Cookies,
) -> OperationResult<Redirect> {
    let mut tx = transaction(&pool).await?;

    let res = new_user_session(&mut tx, None, cookies)
        .await
        .context("failed to create new session")
        .map_err(into_log_server_error!());

    commit_ok(res, tx).await?;

    Ok(query.to_redirect())
}

pub async fn logout(
    State(pool): State<Pool>,
    Query(query): Query<LoginQuery>,
    cookies: Cookies,
) -> OperationResult<Redirect> {
    let mut conn = acquire(&pool).await?;

    if let Some(cookie) = cookies.get(COOKIE_NAME) {
        // delete session if it exists
        if let Ok(session_id) = cookie.value().parse() {
            delete_session(&mut *conn, session_id)
                .await
                .context("Failed to delete session")
                .map_err(into_log_server_error!())?;
        }

        cookies.remove(Cookie::new(COOKIE_NAME, ""));
    }

    Ok(query.to_redirect())
}
