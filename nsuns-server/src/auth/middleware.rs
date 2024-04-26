use anyhow::anyhow;
use axum::{
    extract::State,
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};
use chrono::Utc;
use http::{Request, StatusCode, Uri};
use tower_cookies::Cookies;

use crate::error::{ErrorWithStatus, OperationResult};

use super::token::{create_empty_cookie, JwtKeys, COOKIE_NAME};

pub async fn redirect_on_missing_auth_cookie<B>(
    cookies: Cookies,
    uri: Uri,
    request: Request<B>,
    next: Next<B>,
) -> Response {
    if cookies.get(COOKIE_NAME).is_none() && uri.path() != "/login" {
        Redirect::to("/login").into_response()
    } else if cookies.get(COOKIE_NAME).is_some() && uri.path() == "/login" {
        Redirect::to("/").into_response()
    } else {
        next.run(request).await
    }
}

pub async fn manage_tokens<B>(
    State(keys): State<JwtKeys>,
    cookies: Cookies,
    mut request: Request<B>,
    next: Next<B>,
) -> OperationResult<Response> {
    if let Some(cookie) = cookies.get(COOKIE_NAME) {
        // TODO determine if we should remove the cookie
        let claims = keys.decode(cookie.value())?;

        if Utc::now().gt(&claims.exp) {
            cookies.remove(create_empty_cookie());
            return Err(ErrorWithStatus::new(StatusCode::UNAUTHORIZED, anyhow!("")));
        }

        request.extensions_mut().insert(claims);
    }

    Ok(next.run(request).await)
}
