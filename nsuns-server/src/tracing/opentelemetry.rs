use anyhow::Context;
use opentelemetry::KeyValue;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{
    runtime::Tokio,
    trace::{self, Sampler, Tracer, XrayIdGenerator},
    Resource,
};
use opentelemetry_semantic_conventions as semcov;
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::registry::LookupSpan;

use super::settings::OpenTelemetrySettings;

pub fn layer<S: tracing::Subscriber + for<'span> LookupSpan<'span>>(
    settings: &OpenTelemetrySettings,
) -> anyhow::Result<OpenTelemetryLayer<S, Tracer>> {
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

    Ok(opentelemetry)
}
