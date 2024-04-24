use anyhow::{anyhow, Context};
use axum::{extract::State, middleware::Next, response::Response};
use chrono::Utc;
use http::{Request, StatusCode};
use tower_cookies::{Cookie, Cookies};
use uuid::Uuid;

use crate::{
    error::{ErrorWithStatus, OperationResult},
    into_log_server_error,
};

use super::token::{create_token_cookie, Claims, JwtKeys, COOKIE_NAME};

fn renew_token(
    keys: &JwtKeys,
    cookies: &Cookies,
    expired_claims: &Claims,
    user_id: Uuid,
) -> OperationResult<Claims> {
    let new_claims = Claims::generate(expired_claims.owner_id, Some(user_id));

    let token = keys
        .encode(&new_claims)
        .context("failed to generate new token")
        .map_err(into_log_server_error!())?;

    cookies.add(create_token_cookie(token));

    Ok(new_claims)
}

pub async fn manage_tokens<B>(
    State(keys): State<JwtKeys>,
    cookies: Cookies,
    mut request: Request<B>,
    next: Next<B>,
) -> OperationResult<Response> {
    if let Some(token) = cookies
        .get(COOKIE_NAME)
        .and_then(|cookie| cookie.value_raw())
    {
        let mut claims = keys.decode(token)?;

        if Utc::now().gt(&claims.expiry_date) {
            if let Some(user_id) = claims.user_id {
                // automatically generate a new session if the existing one is expired and not anonymous
                claims = renew_token(&keys, &cookies, &claims, user_id)?;
            } else {
                // expired anonymous user
                cookies.remove(Cookie::named(COOKIE_NAME));
                return Err(ErrorWithStatus::new(StatusCode::UNAUTHORIZED, anyhow!("")));
            }
        }

        request.extensions_mut().insert(claims);
    }

    Ok(next.run(request).await)
}
