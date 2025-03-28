use std::sync::Arc;

use anyhow::anyhow;
use axum::{
    extract::{Request, State},
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};
use chrono::Utc;
use http::{StatusCode, Uri};
use tower_cookies::Cookies;

use crate::{
    acquire,
    db::Pool,
    error::{extract::WithErrorRejection, ErrorWithStatus, OperationResult},
};

use super::token::{create_empty_cookie, Claims, JwtKeys, COOKIE_NAME};

#[derive(Debug, Clone)]
pub struct ClaimsResult(pub Result<Claims, Arc<ErrorWithStatus<anyhow::Error>>>);

pub async fn redirect_on_missing_auth_cookie(
    WithErrorRejection(cookies): WithErrorRejection<Cookies>,
    uri: Uri, // uri extraction is infallible
    request: Request,
    next: Next,
) -> Response {
    if cookies.get(COOKIE_NAME).is_none() && uri.path() != "/login" {
        Redirect::to("/login").into_response()
    } else if cookies.get(COOKIE_NAME).is_some() && uri.path() == "/login" {
        Redirect::to("/").into_response()
    } else {
        next.run(request).await
    }
}

fn unauthorized() -> ErrorWithStatus<anyhow::Error> {
    ErrorWithStatus::new(StatusCode::UNAUTHORIZED, anyhow!("Unauthorized"))
}

#[tracing::instrument(skip_all)]
async fn get_and_validate_token(
    pool: Pool,
    keys: JwtKeys,
    cookies: Cookies,
) -> OperationResult<Claims> {
    let Some(cookie) = cookies.get(COOKIE_NAME) else {
        return Err(unauthorized());
    };

    let claims = match keys.decode(cookie.value()) {
        Ok(claims) => claims,
        Err(err) => {
            // remove the cookie if it failed authentication or is invalid
            let err: ErrorWithStatus<anyhow::Error> = err.into();
            if err.status.is_client_error() {
                cookies.remove(create_empty_cookie());
            }
            return Err(err);
        }
    };

    if Utc::now().gt(&claims.exp) {
        cookies.remove(create_empty_cookie());
        return Err(unauthorized());
    }

    // Verify the token has not been revoked
    let mut conn = acquire!(&pool).await?;

    let stored_claims = Claims::select_one(claims.id, &mut *conn).await?;

    if stored_claims.is_none() {
        cookies.remove(create_empty_cookie());
        return Err(unauthorized());
    }

    Ok(claims)
}

pub async fn manage_tokens(
    State(pool): State<Pool>,
    State(keys): State<JwtKeys>,
    cookies: Cookies,
    mut request: Request,
    next: Next,
) -> OperationResult<Response> {
    let claims_result = get_and_validate_token(pool, keys, cookies).await;

    request
        .extensions_mut()
        .insert::<ClaimsResult>(ClaimsResult(claims_result.map_err(Arc::new)));

    Ok(next.run(request).await)
}
