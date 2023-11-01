use anyhow::Context;
use opentelemetry::KeyValue;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{
    runtime::Tokio,
    trace::{self, Sampler, XrayIdGenerator},
    Resource,
};
use opentelemetry_semantic_conventions as semcov;
use tracing_subscriber::{filter::LevelFilter, prelude::*};

use super::settings::OpenTelemetryFeature;

pub fn setup_tracing(settings: &OpenTelemetryFeature) -> anyhow::Result<()> {
    let format_layer = tracing_subscriber::fmt::layer().compact();

    // use json logs in release builds
    #[cfg(not(debug_assertions))]
    let format_layer = format_layer
        .json()
        .with_span_list(false)
        .with_current_span(false)
        .flatten_event(true);

    let layered = tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .with(format_layer);

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
                    .with_id_generator(XrayIdGenerator::default())
                    .with_max_events_per_span(64)
                    .with_max_attributes_per_span(16)
                    .with_resource(Resource::new(vec![KeyValue::new(
                        semcov::resource::SERVICE_NAME,
                        settings.service_name.clone(),
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
