use serde::Deserialize;

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
