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

use crate::{db, settings::Settings, tracing::global_fields::WithGlobalFields};

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

/// Determine if the span is a database client call
fn db_span_filter(attrs: &tracing::span::Attributes<'_>) -> bool {
    attrs
        .metadata()
        .target()
        .ends_with(db::tracing::TRACING_TARGET_SUFFIX)
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

    let telemetry_layer =
        if let OpenTelemetryFeature::Enabled(settings) = &log_settings.opentelemetry {
            Some(otel_layer(settings)?)
        } else {
            None
        };

    let connection_string = format!(
        "Server={server};Database={db};Uid={user};MaximumPoolSize={pool_sz};",
        server = settings.database.host,
        db = settings.database.database,
        user = settings.database.username,
        pool_sz = settings.database.max_connections
    );

    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(telemetry_layer)
        .with_global_fields_filtered(
            [
                ("db.name", settings.database.database.clone()),
                ("db.user", settings.database.username.clone()),
                ("db.connection_string", connection_string),
                ("server.address", settings.database.host.clone()),
            ],
            db_span_filter,
        )
        // separate layer for different type
        .with_global_fields_filtered([("server.port", settings.database.port)], db_span_filter)
        .with(
            tracing_subscriber::EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .parse_lossy(&log_settings.directive),
        )
        .try_init()?;

    Ok(())
}
