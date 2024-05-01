use std::time::Duration;

use config::builder::BuilderState;
use opentelemetry_otlp::{OTEL_EXPORTER_OTLP_ENDPOINT, OTEL_EXPORTER_OTLP_TIMEOUT};
use opentelemetry_sdk::trace::{BatchConfig, SpanLimits};
use serde::Deserialize;

use crate::{
    feature::{Feature, ENABLED_KEY},
    settings::{CustomizeConfigBuilder, SetEnvOverride},
};

#[must_use]
pub fn default_json_format() -> bool {
    true
}

#[must_use]
pub fn default_directive() -> String {
    "info".to_string()
}

#[derive(Debug, Deserialize)]
pub struct LogSettings {
    /// Use json-formatted logs to stdout
    #[serde(default = "default_json_format")]
    pub json: bool,
    /// Logging directive, e.g. "info" to log every crate at INFO level.
    #[serde(default = "default_directive")]
    pub directive: String,
    #[serde(default)]
    pub opentelemetry: OpenTelemetryFeature,
}

impl Default for LogSettings {
    fn default() -> Self {
        Self {
            json: default_json_format(),
            directive: default_directive(),
            opentelemetry: Feature::default(),
        }
    }
}

impl<S: BuilderState> CustomizeConfigBuilder<S> for LogSettings {
    fn customize_builder(
        builder: config::ConfigBuilder<S>,
        prefix: &str,
    ) -> config::ConfigBuilder<S> {
        OpenTelemetryFeature::customize_builder(builder, &format!("{prefix}.opentelemetry"))
            .set_env_override_unwrap(&format!("{prefix}.directive"), "RUST_LOG")
            .set_env_override_unwrap(&format!("{prefix}.json"), "JSON_LOG")
    }
}

#[must_use]
pub fn default_max_queue_size() -> usize {
    2048
}

#[must_use]
pub fn default_scheduled_delay() -> Duration {
    Duration::from_secs(5)
}

#[must_use]
pub fn default_max_export_batch_size() -> usize {
    512
}

#[must_use]
pub fn default_max_export_timeout() -> Duration {
    Duration::from_secs(30)
}

#[must_use]
pub fn default_max_concurrent_exports() -> usize {
    1
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct SpanBatchSettings {
    /// The maximum queue size to buffer spans for delayed processing. If the
    /// queue gets full it drops the spans. The default value of is 2048.
    #[serde(default = "default_max_queue_size")]
    pub max_queue_size: usize,

    /// The delay interval in milliseconds between two consecutive processing
    /// of batches. The default value is 5 seconds.
    #[serde(default = "default_scheduled_delay")]
    #[serde(with = "crate::serde_duration")]
    pub scheduled_delay: Duration,

    /// The maximum number of spans to process in a single batch. If there are
    /// more than one batch worth of spans then it processes multiple batches
    /// of spans one batch after the other without any delay. The default value
    /// is 512.
    #[serde(default = "default_max_export_batch_size")]
    pub max_export_batch_size: usize,

    /// The maximum duration to export a batch of data.
    #[serde(default = "default_max_export_timeout")]
    #[serde(with = "crate::serde_duration")]
    pub max_export_timeout: Duration,

    /// Maximum number of concurrent exports
    ///
    /// Limits the number of spawned tasks for exports and thus memory consumed
    /// by an exporter. A value of 1 will cause exports to be performed
    /// synchronously on the BatchSpanProcessor task.
    #[serde(default = "default_max_concurrent_exports")]
    pub max_concurrent_exports: usize,
}

impl Default for SpanBatchSettings {
    fn default() -> Self {
        Self {
            max_queue_size: default_max_queue_size(),
            scheduled_delay: default_scheduled_delay(),
            max_export_batch_size: default_max_export_batch_size(),
            max_export_timeout: default_max_export_timeout(),
            max_concurrent_exports: default_max_concurrent_exports(),
        }
    }
}

impl From<&SpanBatchSettings> for BatchConfig {
    fn from(settings: &SpanBatchSettings) -> Self {
        BatchConfig::default()
            .with_max_queue_size(settings.max_queue_size)
            .with_scheduled_delay(settings.scheduled_delay)
            .with_max_export_batch_size(settings.max_export_batch_size)
            .with_max_export_timeout(settings.max_export_timeout)
            .with_max_concurrent_exports(settings.max_concurrent_exports)
    }
}

#[must_use]
pub fn default_span_limit() -> u32 {
    128
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct SpanSettings {
    /// The max events that can be added to a `Span`.
    #[serde(default = "default_span_limit")]
    pub max_events_per_span: u32,
    /// The max attributes that can be added to a `Span`.
    #[serde(default = "default_span_limit")]
    pub max_attributes_per_span: u32,
    /// The max links that can be added to a `Span`.
    #[serde(default = "default_span_limit")]
    pub max_links_per_span: u32,
    /// The max attributes that can be added into an `Event`
    #[serde(default = "default_span_limit")]
    pub max_attributes_per_event: u32,
    /// The max attributes that can be added into a `Link`
    #[serde(default = "default_span_limit")]
    pub max_attributes_per_link: u32,
}

impl Default for SpanSettings {
    fn default() -> Self {
        Self {
            max_events_per_span: default_span_limit(),
            max_attributes_per_span: default_span_limit(),
            max_links_per_span: default_span_limit(),
            max_attributes_per_event: default_span_limit(),
            max_attributes_per_link: default_span_limit(),
        }
    }
}

impl From<&SpanSettings> for SpanLimits {
    fn from(settings: &SpanSettings) -> Self {
        Self {
            max_events_per_span: settings.max_events_per_span,
            max_attributes_per_span: settings.max_attributes_per_span,
            max_links_per_span: settings.max_links_per_span,
            max_attributes_per_event: settings.max_attributes_per_event,
            max_attributes_per_link: settings.max_attributes_per_link,
        }
    }
}

#[must_use]
pub fn default_exporter_host() -> String {
    // jaeger default port
    "http://localhost:4317".to_string()
}

#[must_use]
pub fn default_exporter_timeout() -> Duration {
    Duration::from_secs(3)
}

#[must_use]
pub fn default_service_name() -> String {
    env!("CARGO_PKG_NAME").to_string()
}

#[must_use]
pub fn default_sample_rate() -> f64 {
    0.1
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct OpenTelemetrySettings {
    #[serde(default = "default_exporter_host")]
    pub exporter_host: String,
    #[serde(default = "default_exporter_timeout")]
    #[serde(with = "crate::serde_duration")]
    pub exporter_timeout: Duration,
    #[serde(default = "default_service_name")]
    pub service_name: String,
    #[serde(default)]
    pub batch: SpanBatchSettings,
    #[serde(default)]
    pub span_limits: SpanSettings,
    #[serde(default = "default_sample_rate")]
    pub sample_rate: f64,
}

impl Default for OpenTelemetrySettings {
    fn default() -> Self {
        Self {
            exporter_host: default_exporter_host(),
            exporter_timeout: default_exporter_timeout(),
            service_name: default_service_name(),
            batch: SpanBatchSettings::default(),
            span_limits: SpanSettings::default(),
            sample_rate: default_sample_rate(),
        }
    }
}

pub type OpenTelemetryFeature = Feature<OpenTelemetrySettings>;

impl Default for OpenTelemetryFeature {
    fn default() -> Self {
        Self::Disabled
    }
}

impl<S: BuilderState> CustomizeConfigBuilder<S> for OpenTelemetrySettings {
    fn customize_builder(
        builder: config::ConfigBuilder<S>,
        prefix: &str,
    ) -> config::ConfigBuilder<S> {
        builder
            .set_env_override_unwrap(&format!("{prefix}.{ENABLED_KEY}"), "OTEL_ENABLED")
            .set_env_override_unwrap(
                &format!("{prefix}.exporter_host"),
                OTEL_EXPORTER_OTLP_ENDPOINT,
            )
            .set_env_override_unwrap(
                &format!("{prefix}.exporter_timeout"),
                OTEL_EXPORTER_OTLP_TIMEOUT,
            )
            .set_env_override_unwrap(&format!("{prefix}.service_name"), "OTEL_SERVICE_NAME")
    }
}
