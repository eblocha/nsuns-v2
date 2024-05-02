use opentelemetry_api::global;
use tracing_subscriber::{filter::LevelFilter, fmt, prelude::*};

use crate::settings::Settings;

use super::{opentelemetry, settings::OpenTelemetryFeature};

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
    let fmt_layer = if settings.logging.json {
        fmt::layer()
            .json()
            .with_span_list(false)
            .with_current_span(false)
            .flatten_event(true)
            .boxed()
    } else {
        fmt::layer().pretty().boxed()
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

    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(telemetry_layer)
        .with(
            tracing_subscriber::EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .parse_lossy(&settings.logging.directive),
        )
        .try_init()?;

    Ok(trace_guard)
}
