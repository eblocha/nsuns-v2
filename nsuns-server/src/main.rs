use anyhow::{Context, Result};
use nsuns_server::{api_server, error::LogError, settings::Settings};
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

    api_server(&settings).await.log_error()
}
