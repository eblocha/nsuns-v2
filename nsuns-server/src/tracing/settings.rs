use std::time::Duration;

use config::builder::BuilderState;
use opentelemetry_otlp::{OTEL_EXPORTER_OTLP_ENDPOINT, OTEL_EXPORTER_OTLP_TIMEOUT};
use serde::Deserialize;

use crate::{
    feature::Feature,
    settings::{CustomizeConfigBuilder, SetEnvOverride},
};

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
