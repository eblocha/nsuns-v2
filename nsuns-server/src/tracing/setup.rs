use anyhow::Context;
use opentelemetry::KeyValue;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{
    runtime::Tokio,
    trace::{self, RandomIdGenerator, Sampler},
    Resource,
};
use opentelemetry_semantic_conventions as semcov;
use tracing_subscriber::prelude::*;

use super::settings::OpenTelemetryFeature;

pub fn setup_tracing(settings: &OpenTelemetryFeature) -> anyhow::Result<()> {
    let layered = tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .with(tracing_subscriber::fmt::layer());

    if let OpenTelemetryFeature::Enabled(settings) = settings {
        let otlp_exporter = opentelemetry_otlp::new_exporter()
            .tonic()
            .with_endpoint(&settings.exporter_host)
            .with_timeout(settings.exporter_timeout);

        let tracer = opentelemetry_otlp::new_pipeline()
            .tracing()
            .with_exporter(otlp_exporter)
            .with_trace_config(
                trace::config()
                    .with_sampler(Sampler::AlwaysOn)
                    .with_id_generator(RandomIdGenerator::default())
                    .with_max_events_per_span(64)
                    .with_max_attributes_per_span(16)
                    .with_resource(Resource::new(vec![KeyValue::new(
                        semcov::resource::SERVICE_NAME,
                        "nsuns",
                    )])),
            )
            .install_batch(Tokio)
            .with_context(|| "failed to install otel tracer")?;

        let opentelemetry = tracing_opentelemetry::layer().with_tracer(tracer);

        layered.with(opentelemetry).try_init()
    } else {
        layered.try_init()
    }
    .with_context(|| "failed to init tracing subscriber")?;

    Ok(())
}
