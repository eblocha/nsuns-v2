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

fn service_version() -> String {
    let pkg_version = env!("CARGO_PKG_VERSION");

    match option_env!("NSUNS_BUILD_GIT_HASH") {
        Some(hash) => format!("{pkg_version}-{hash}"),
        None => pkg_version.to_owned(),
    }
}

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
        .with_batch_config((&settings.batch).into())
        .with_trace_config(
            trace::config()
                .with_sampler(Sampler::TraceIdRatioBased(settings.sample_rate))
                .with_span_limits((&settings.span_limits).into())
                .with_id_generator(XrayIdGenerator::default())
                .with_resource(Resource::new(vec![
                    KeyValue::new(
                        semcov::resource::SERVICE_NAME,
                        settings.service_name.clone(),
                    ),
                    KeyValue::new(semcov::resource::SERVICE_VERSION, service_version()),
                ])),
        )
        .install_batch(Tokio)
        .context("failed to install otel tracer")?;

    let opentelemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    Ok(opentelemetry)
}
