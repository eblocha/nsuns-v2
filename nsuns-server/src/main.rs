use anyhow::{Context, Result};
use nsuns_server::{
    log_error, metrics::server as metrics_server, server, settings::Settings,
    tracing::setup::setup_tracing,
};

#[tokio::main]
async fn main() -> Result<()> {
    let settings = Settings::new()
        .with_context(|| "failed to load settings")
        .map_err(log_error!())?;

    let _guard = setup_tracing(&settings)?;

    tokio::try_join!(
        server::run(&settings),
        metrics_server::run(&settings.metrics)
    )
    .map_err(log_error!())?;

    Ok(())
}
