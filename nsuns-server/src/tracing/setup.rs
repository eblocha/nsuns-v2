use anyhow::Context;
use tracing_subscriber::prelude::*;

use super::settings::OpenTelemetryFeature;

pub fn setup_tracing(settings: &OpenTelemetryFeature) -> anyhow::Result<()> {
    let layered = tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .with(tracing_subscriber::fmt::layer());

    if settings.is_enabled() {
        let tracer = opentelemetry_jaeger::new_agent_pipeline()
            .with_service_name("nsuns")
            .install_simple()
            .with_context(|| "failed to install otel tracer")?;

        let opentelemetry = tracing_opentelemetry::layer().with_tracer(tracer);

        layered.with(opentelemetry).try_init()
    } else {
        layered.try_init()
    }
    .with_context(|| "failed to init tracing subscriber")?;

    Ok(())
}
