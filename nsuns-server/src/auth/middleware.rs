use anyhow::Context;
use axum::{
    extract::State,
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};
use chrono::Utc;
use http::Request;
use tower_cookies::Cookies;

use crate::{db::Pool, error::OperationResult, into_log_server_error};

use super::session::{get_session, get_session_id_from_cookies};

pub async fn manage_session<B>(
    State(pool): State<Pool>,
    cookies: Cookies,
    mut request: Request<B>,
    next: Next<B>,
) -> OperationResult<Response> {
    // TODO redirect if there is no session cookie and the request is non-api

    // get the session id from the cookie and insert it into the request extensions
    if let Some(session_id) = get_session_id_from_cookies(&cookies) {
        let session = get_session::<()>(&pool, session_id)
            .await
            .context("failed to get session")
            .map_err(into_log_server_error!())?;

        if let Some(session) = session {
            // TODO automatically generate a new session if the existing one is expired and not anonymous

            // redirect to login page if session has expired
            // TODO inclue ?next=... query
            // TODO remove cookie
            // TODO delete expired session
            if Utc::now().gt(&session.expiry_date) {
                return Ok(Redirect::to("/login").into_response());
            }

            request.extensions_mut().insert(session);
        }
    }

    Ok(next.run(request).await)
}
