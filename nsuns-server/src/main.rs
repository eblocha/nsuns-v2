use anyhow::{Context, Result};
use nsuns_server::{
    api_server,
    error::LogError,
    metrics::metrics_server,
    settings::{MetricsFeature, Settings},
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

    let api_future = api_server(&settings);

    if let MetricsFeature::Enabled(ref metrics_settings) = settings.metrics {
        tokio::try_join!(api_future, metrics_server(metrics_settings)).log_error()?;
        Ok(())
    } else {
        api_future.await.log_error()
    }
}
