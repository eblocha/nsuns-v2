use serde::Deserialize;

use crate::feature::Feature;

#[derive(Debug, Deserialize)]
pub struct OpenTelemetrySettings {}

pub type OpenTelemetryFeature = Feature<OpenTelemetrySettings>;

impl Default for OpenTelemetryFeature {
    fn default() -> Self {
        Feature::Enabled(OpenTelemetrySettings {})
    }
}
