use opentelemetry_api::global;
use tracing_subscriber::{filter::LevelFilter, fmt, prelude::*};

use crate::{db, settings::Settings, tracing::global_fields::WithGlobalFields};

use super::{opentelemetry, settings::OpenTelemetryFeature};

/// Determine if the span is a database client call
fn db_span_filter(attrs: &tracing::span::Attributes<'_>) -> bool {
    attrs
        .metadata()
        .target()
        .ends_with(db::tracing::TRACING_TARGET_SUFFIX)
}

pub struct TraceGuard {
    opentelemetry: bool,
}

impl Drop for TraceGuard {
    fn drop(&mut self) {
        if self.opentelemetry {
            global::shutdown_tracer_provider();
        }
    }
}

/// Set up tracing for the application.
///
/// NOTE: this will configure tracing for the process _globally_, so do not call it outside the entry thread.
#[must_use = "Telemetry will be de-initialized on guard drop"]
pub fn setup_tracing(settings: &Settings) -> anyhow::Result<TraceGuard> {
    let fmt_layer = match settings.logging.json {
        true => fmt::layer()
            .json()
            .with_span_list(false)
            .with_current_span(false)
            .flatten_event(true)
            .boxed(),
        false => fmt::layer().pretty().boxed(),
    };

    let telemetry_layer =
        if let OpenTelemetryFeature::Enabled(settings) = &settings.logging.opentelemetry {
            Some(opentelemetry::layer(settings)?)
        } else {
            None
        };

    let trace_guard = TraceGuard {
        opentelemetry: settings.logging.opentelemetry.is_enabled(),
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
                .parse_lossy(&settings.logging.directive),
        )
        .try_init()?;

    Ok(trace_guard)
}
