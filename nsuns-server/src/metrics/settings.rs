use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MetricsSettings {
    #[serde(default)]
    pub port: u16,
    #[serde(default)]
    pub path: String,
}

impl Default for MetricsSettings {
    fn default() -> Self {
        Self {
            port: 9100,
            path: "/metrics".to_string(),
        }
    }
}
