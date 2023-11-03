use std::time::Duration;

use config::builder::BuilderState;
use opentelemetry_otlp::{OTEL_EXPORTER_OTLP_ENDPOINT, OTEL_EXPORTER_OTLP_TIMEOUT};
use serde::Deserialize;

use crate::{
    feature::{Feature, ENABLED_KEY},
    settings::{CustomizeConfigBuilder, SetEnvOverride},
};

pub fn default_json_format() -> bool {
    true
}

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
            opentelemetry: Default::default(),
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

pub fn default_exporter_host() -> String {
    // jaeger default port
    "http://localhost:4317".to_string()
}

pub fn default_exporter_timeout() -> Duration {
    Duration::from_secs(3)
}

pub fn default_service_name() -> String {
    "nsuns".to_string()
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
}

impl Default for OpenTelemetrySettings {
    fn default() -> Self {
        Self {
            exporter_host: default_exporter_host(),
            exporter_timeout: default_exporter_timeout(),
            service_name: default_service_name(),
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
