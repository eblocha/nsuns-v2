use anyhow::{Context, Result};
use nsuns_server::{
    log_error, metrics::server as metrics_server, server, settings::Settings,
    tracing::setup::setup_tracing,
};
use opentelemetry::global;

#[tokio::main]
async fn main() -> Result<()> {
    let settings = Settings::new()
        .with_context(|| "failed to load settings")
        .map_err(log_error!())?;

    setup_tracing(&settings)?;

    tracing::debug!("loaded configuration:\n{:#?}", settings);

    tokio::try_join!(
        server::run(&settings),
        metrics_server::run(&settings.metrics)
    )
    .map_err(log_error!())?;

    global::shutdown_tracer_provider();

    Ok(())
}
