use std::time::Duration;

use anyhow::Context;
use chrono::Utc;
use secrecy::ExposeSecret;
use sqlx::Connection;
use tokio::time::MissedTickBehavior;
use tokio_util::sync::CancellationToken;
use tracing::Instrument;

use crate::{
    db::tracing::InstrumentExecutor, db_span, into_log_server_error, settings::Settings,
    shutdown::shutdown_signal,
};

pub async fn run(settings: &Settings) -> anyhow::Result<()> {
    tracing::info!("starting background cleanup task");

    let mut interval = tokio::time::interval(Duration::from_secs(3600));
    interval.set_missed_tick_behavior(MissedTickBehavior::Delay);

    let cancellation_token = CancellationToken::new();
    let child_cancellation_token = cancellation_token.clone();

    tokio::spawn(async move {
        shutdown_signal().await;
        child_cancellation_token.cancel();
    });

    let url = format!(
        "postgres://{}:{}@{}:{}/{}",
        settings.database.username,
        settings.database.password.expose_secret(),
        settings.database.host,
        settings.database.port,
        settings.database.database
    );

    loop {
        tokio::select! {
            _ = interval.tick() => {},
            _ = cancellation_token.cancelled() => {
                break;
            }
        };

        run_cleanup(&url).await;
    }

    Ok(())
}

#[tracing::instrument(skip_all)]
async fn run_cleanup(url: &str) {
    tracing::info!("starting scheduled cleanup");

    let mut conn = match sqlx::PgConnection::connect(url)
        .instrument(db_span!("acquire connection"))
        .await
        .context("failed to establish a connection")
    {
        Ok(conn) => conn,
        Err(e) => {
            tracing::error!("{e:?}");
            return;
        }
    };

    let now = Utc::now();

    let result_sessions = sqlx::query("DELETE FROM sessions WHERE exp < $1")
        .bind(now)
        .execute((&mut conn).instrument_executor(db_span!("DELETE FROM", "sessions")))
        .await
        .context("failed to delete expired sessions")
        .map_err(into_log_server_error!());

    if let Ok(ref result) = result_sessions {
        tracing::info!("Removed {} expired sessions", result.rows_affected());
    }

    let result_owners = sqlx::query("DELETE FROM owners WHERE expiry_date < $1")
        .bind(now)
        .execute((&mut conn).instrument_executor(db_span!("DELETE FROM", "owners")))
        .await
        .context("failed to delete expired owners")
        .map_err(into_log_server_error!());

    if let Ok(ref result) = result_owners {
        tracing::info!("Removed {} expired users", result.rows_affected());
    }
}
