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
use tracing_subscriber::{filter::LevelFilter, fmt, prelude::*, registry::LookupSpan};

use crate::{db::tracing::layer::WithGlobalFields, settings::Settings};

use super::settings::{LogSettings, OpenTelemetryFeature, OpenTelemetrySettings};

fn otel_layer<S: tracing::Subscriber + for<'span> LookupSpan<'span>>(
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

pub fn setup_tracing(log_settings: &LogSettings, settings: &Settings) -> anyhow::Result<()> {
    let fmt_layer = match log_settings.json {
        true => fmt::layer()
            .json()
            .with_span_list(false)
            .with_current_span(false)
            .flatten_event(true)
            .boxed(),
        false => fmt::layer().pretty().boxed(),
    };

    let registry = tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .parse_lossy(&log_settings.directive),
        )
        .with(fmt_layer);

    let connection_string = format!(
        "Server={server};Database={db};Uid={user};MaximumPoolSize={pool_sz};",
        server = settings.database.host,
        db = settings.database.database,
        user = settings.database.username,
        pool_sz = settings.database.max_connections
    );

    let global_fields = [
        ("db.name", settings.database.database.clone()),
        ("db.user", settings.database.username.clone()),
        ("db.connection_string", connection_string),
        ("server.address", settings.database.host.clone()),
    ];

    if let OpenTelemetryFeature::Enabled(settings) = &log_settings.opentelemetry {
        registry
            .with(otel_layer(settings)?)
            .with_global_fields_filtered(global_fields, |attrs: &tracing::span::Attributes<'_>| {
                attrs.metadata().target() == "nsuns_server::db"
            })
            .try_init()
    } else {
        registry.with_global_fields(global_fields).try_init()
    }
    .with_context(|| "failed to init tracing subscriber")?;

    Ok(())
}
