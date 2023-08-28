use anyhow::{Context, Result};
use nsuns_server::{
    error::LogError, metrics::server as metrics_server, server, settings::Settings,
};
use tracing_subscriber::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let settings = Settings::new()
        .with_context(|| "failed to load settings")
        .log_error()?;

    tokio::try_join!(
        server::run(&settings),
        metrics_server::run(&settings.metrics)
    )
    .log_error()?;
    Ok(())
}
