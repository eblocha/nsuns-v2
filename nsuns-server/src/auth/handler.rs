use anyhow::{anyhow, Context};
use axum::{extract::State, response::IntoResponse, Json};
use axum_extra::{
    headers::{authorization::Basic, Authorization},
    TypedHeader,
};
use http::StatusCode;
use sqlx::{Executor, Transaction};
use tower_cookies::Cookies;

use crate::{
    db::{transaction::commit_ok, Pool, DB},
    error::{extract::WithErrorRejection, ErrorWithStatus, OperationResult},
    into_log_server_error, transaction,
};

use super::{
    token::{
        create_empty_cookie, create_new_expiry_date, create_token_cookie,
        decode_claims_from_cookies, Claims, JwtKeys, OwnerId,
    },
    user::{
        authenticate, create_anonymous_user, delete_owner, select_owner_expiry,
        select_user_info_by_owner_id, AgentInfo, AnonymousInfo,
    },
};

async fn login_user(
    tx: &mut Transaction<'_, DB>,
    keys: &JwtKeys,
    auth: Basic,
    cookies: Cookies,
) -> OperationResult<()> {
    // authenticate user
    let Some(user) = authenticate(&mut **tx, auth)
        .await
        .context("Failed to authenticate")
        .map_err(into_log_server_error!())?
    else {
        return Err(ErrorWithStatus::new(
            StatusCode::UNAUTHORIZED,
            anyhow!("Bad credentials"),
        ));
    };

    let claims = Claims::insert_one(&mut **tx, user.owner_id, Some(user.id), None).await?;
    let token = keys.encode(&claims).map_err(into_log_server_error!())?;
    let cookie = create_token_cookie(token);

    delete_owner_if_anonymous(Some(claims), &mut **tx).await?;

    cookies.add(cookie);

    Ok(())
}

#[tracing::instrument(skip_all)]
pub async fn login(
    State(pool): State<Pool>,
    State(keys): State<JwtKeys>,
    WithErrorRejection(TypedHeader(Authorization(creds))): WithErrorRejection<
        TypedHeader<Authorization<Basic>>,
    >,
    WithErrorRejection(cookies): WithErrorRejection<Cookies>,
) -> OperationResult<StatusCode> {
    let mut tx = transaction!(&pool).await?;

    let res = login_user(&mut tx, &keys, creds, cookies).await;

    commit_ok(res, tx).await?;

    Ok(StatusCode::NO_CONTENT)
}

async fn login_anonymous(
    tx: &mut Transaction<'_, DB>,
    keys: &JwtKeys,
    cookies: Cookies,
) -> OperationResult<()> {
    delete_owner_if_anonymous(decode_claims_from_cookies(keys, &cookies), &mut **tx).await?;

    let exp = create_new_expiry_date();

    let owner_id = create_anonymous_user(&mut **tx, exp)
        .await
        .context("failed to create new anonymous owner")
        .map_err(into_log_server_error!())?;

    let claims = Claims::insert_one(&mut **tx, owner_id.as_uuid(), None, Some(exp)).await?;
    let token = keys.encode(&claims).map_err(into_log_server_error!())?;
    let cookie = create_token_cookie(token);

    cookies.add(cookie);

    Ok(())
}

#[tracing::instrument(skip_all)]
pub async fn anonymous(
    State(pool): State<Pool>,
    State(keys): State<JwtKeys>,
    WithErrorRejection(cookies): WithErrorRejection<Cookies>,
) -> OperationResult<StatusCode> {
    let mut tx = transaction!(&pool).await?;

    let res = login_anonymous(&mut tx, &keys, cookies).await;

    commit_ok(res, tx).await?;

    Ok(StatusCode::NO_CONTENT)
}

async fn delete_owner_if_anonymous(
    claims: Option<Claims>,
    executor: impl Executor<'_, Database = DB>,
) -> OperationResult<()> {
    if let Some(claims) = claims {
        if claims.user_id.is_none() {
            delete_owner(executor, claims.owner_id)
                .await
                .context("failed to delete anonymous owner")
                .map_err(into_log_server_error!())?;
        }
    }

    Ok(())
}

async fn revoke_and_logout(claims: Claims, tx: &mut Transaction<'_, DB>) -> OperationResult<()> {
    if claims.revoke(&mut **tx).await?.is_some() {
        // Only delete the owner if the token was not revoked.
        // This prevents revoked tokens from deleting anonymous users.
        delete_owner_if_anonymous(Some(claims), &mut **tx).await?;
    }
    Ok(())
}

#[tracing::instrument(skip_all)]
pub async fn logout(
    State(pool): State<Pool>,
    State(keys): State<JwtKeys>,
    WithErrorRejection(cookies): WithErrorRejection<Cookies>,
) -> OperationResult<StatusCode> {
    if let Some(claims) = decode_claims_from_cookies(&keys, &cookies) {
        let mut tx = transaction!(&pool).await?;
        let res = revoke_and_logout(claims, &mut tx).await;
        commit_ok(res, tx).await?;
    }

    cookies.remove(create_empty_cookie());

    Ok(StatusCode::NO_CONTENT)
}

#[tracing::instrument(skip_all)]
pub async fn agent_info(State(pool): State<Pool>, owner_id: OwnerId) -> impl IntoResponse {
    let mut tx = transaction!(&pool).await?;

    let user_option = select_user_info_by_owner_id(owner_id, &mut *tx)
        .await
        .context("failed to fetch user info")
        .map_err(into_log_server_error!())?;

    if let Some(user) = user_option {
        Ok(Json(AgentInfo::User(user)))
    } else if let Some(expiry_date) = select_owner_expiry(owner_id, &mut *tx)
        .await
        .context("failed to fetch anonymous owner info")
        .map_err(into_log_server_error!())?
    {
        Ok(Json(AgentInfo::Anonymous(AnonymousInfo { expiry_date })))
    } else {
        let e = ErrorWithStatus::new(StatusCode::NOT_FOUND, anyhow!("No user info found"));
        Err(e)
    }
}
