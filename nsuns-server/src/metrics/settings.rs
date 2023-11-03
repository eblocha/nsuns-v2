use config::builder::BuilderState;
use serde::Deserialize;

use crate::{
    feature::{Feature, ENABLED_KEY},
    settings::{CustomizeConfigBuilder, SetEnvOverride},
};

fn default_metrics_port() -> u16 {
    9100
}

fn default_metrics_path() -> String {
    "/metrics".to_string()
}

#[derive(Debug, Deserialize)]
pub struct MetricsSettings {
    #[serde(default = "default_metrics_port")]
    pub port: u16,
    #[serde(default = "default_metrics_path")]
    pub path: String,
}

impl Default for MetricsSettings {
    fn default() -> Self {
        Self {
            port: default_metrics_port(),
            path: default_metrics_path(),
        }
    }
}

pub type MetricsFeature = Feature<MetricsSettings>;

impl Default for MetricsFeature {
    fn default() -> Self {
        Self::Enabled(Default::default())
    }
}

impl<S: BuilderState> CustomizeConfigBuilder<S> for MetricsSettings {
    fn customize_builder(
        builder: config::ConfigBuilder<S>,
        prefix: &str,
    ) -> config::ConfigBuilder<S> {
        builder.set_env_override_unwrap(&format!("{prefix}.{ENABLED_KEY}"), "METRICS_ENABLED")
    }
}
