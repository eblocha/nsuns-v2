use anyhow::{anyhow, Context};
use axum::{
    extract::State,
    headers::{authorization::Basic, Authorization},
    TypedHeader,
};
use http::StatusCode;
use sqlx::Transaction;
use tower_cookies::{Cookie, Cookies};
use transaction::{acquire, commit_ok};

use crate::{
    db::{transaction, Pool, DB},
    error::{ErrorWithStatus, OperationResult},
    into_log_server_error,
};

use super::{
    token::{create_new_expiry_date, create_token_cookie, Claims, JwtKeys, COOKIE_NAME},
    user::{authenticate, create_anonymous_user, delete_owner},
};

async fn login_user(
    tx: &mut Transaction<'_, DB>,
    keys: &JwtKeys,
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

    let claims = Claims::generate(user.owner_id, Some(user.id));
    let token = keys.encode(&claims).map_err(into_log_server_error!())?;
    let cookie = create_token_cookie(token);

    cookies.add(cookie);

    Ok(())
}

#[tracing::instrument(skip_all)]
pub async fn login(
    State(pool): State<Pool>,
    State(keys): State<JwtKeys>,
    TypedHeader(Authorization(creds)): TypedHeader<Authorization<Basic>>,
    cookies: Cookies,
) -> OperationResult<()> {
    let mut tx = transaction(&pool).await?;

    let res = login_user(&mut tx, &keys, creds, cookies).await;

    commit_ok(res, tx).await?;

    Ok(())
}

async fn login_anonymous(
    tx: &mut Transaction<'_, DB>,
    keys: &JwtKeys,
    cookies: Cookies,
) -> OperationResult<()> {
    let exp = create_new_expiry_date();

    let owner_id = create_anonymous_user(&mut **tx, exp)
        .await
        .context("failed to create new anonymous owner")
        .map_err(into_log_server_error!())?;

    let claims = Claims {
        owner_id,
        user_id: None,
        exp,
    };
    let token = keys.encode(&claims).map_err(into_log_server_error!())?;
    let cookie = create_token_cookie(token);

    cookies.add(cookie);

    Ok(())
}

#[tracing::instrument(skip_all)]
pub async fn anonymous(
    State(pool): State<Pool>,
    State(keys): State<JwtKeys>,
    cookies: Cookies,
) -> OperationResult<()> {
    let mut tx = transaction(&pool).await?;

    let res = login_anonymous(&mut tx, &keys, cookies).await;

    commit_ok(res, tx).await?;

    Ok(())
}

#[tracing::instrument(skip_all)]
pub async fn logout(
    State(pool): State<Pool>,
    State(keys): State<JwtKeys>,
    cookies: Cookies,
) -> OperationResult<()> {
    cookies.remove(Cookie::named(COOKIE_NAME));

    if let Some(claims) = cookies
        .get(COOKIE_NAME)
        .and_then(|cookie| cookie.value_raw())
        .and_then(|token| keys.decode(token).ok())
    {
        // delete owner from db if it is anonymous
        if claims.user_id.is_none() {
            let mut conn = acquire(&pool).await?;
            delete_owner(&mut *conn, claims.owner_id)
                .await
                .context("failed to delete anonymous owner")
                .map_err(into_log_server_error!())?;
        }
    }

    Ok(())
}
