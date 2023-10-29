use anyhow::{Context, Result};
use nsuns_server::{
    error::LogError, metrics::server as metrics_server, server, settings::Settings,
};
use opentelemetry::global;
use tracing_subscriber::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let tracer = opentelemetry_jaeger::new_agent_pipeline()
        .with_service_name("nsuns")
        .install_simple()
        .with_context(|| "failed to install otel tracer")?;

    let opentelemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .with(opentelemetry)
        .try_init()
        .with_context(|| "failed to init tracing subscriber")?;

    let settings = Settings::new()
        .with_context(|| "failed to load settings")
        .log_error()?;

    tokio::try_join!(
        server::run(&settings),
        metrics_server::run(&settings.metrics)
    )
    .log_error()?;

    global::shutdown_tracer_provider();

    Ok(())
}
