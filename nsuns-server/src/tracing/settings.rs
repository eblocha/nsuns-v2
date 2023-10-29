use std::time::Duration;

use serde::Deserialize;

use crate::feature::Feature;

pub fn default_exporter_host() -> String {
    // jaeger default port
    "http://localhost:4317".to_string()
}

pub fn default_exporter_timeout() -> Duration {
    Duration::from_secs(3)
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct OpenTelemetrySettings {
    #[serde(default = "default_exporter_host")]
    pub exporter_host: String,
    #[serde(default = "default_exporter_timeout")]
    pub exporter_timeout: Duration,
}

impl Default for OpenTelemetrySettings {
    fn default() -> Self {
        Self {
            exporter_host: default_exporter_host(),
            exporter_timeout: default_exporter_timeout(),
        }
    }
}

pub type OpenTelemetryFeature = Feature<OpenTelemetrySettings>;

impl Default for OpenTelemetryFeature {
    fn default() -> Self {
        Self::Disabled
    }
}
