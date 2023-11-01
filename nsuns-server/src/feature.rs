use config::{builder::BuilderState, ConfigBuilder};
use serde::Deserialize;

use crate::settings::CustomizeConfigBuilder;

#[derive(Debug, Deserialize)]
pub enum Feature<T> {
    #[serde(rename = "enabled")]
    Enabled(T),
    #[serde(rename = "disabled")]
    Disabled,
}

impl<T> From<Option<T>> for Feature<T> {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(value) => Self::Enabled(value),
            None => Self::Disabled,
        }
    }
}

impl<T> From<Feature<T>> for Option<T> {
    fn from(value: Feature<T>) -> Self {
        match value {
            Feature::Enabled(value) => Some(value),
            Feature::Disabled => None,
        }
    }
}

impl<T> Feature<T> {
    pub fn is_enabled(&self) -> bool {
        matches!(*self, Feature::Enabled(_))
    }
}

impl<S: BuilderState, T: CustomizeConfigBuilder<S>> CustomizeConfigBuilder<S> for Feature<T> {
    fn customize_builder(builder: ConfigBuilder<S>, prefix: &str) -> ConfigBuilder<S> {
        T::customize_builder(builder, &format!("{prefix}.enabled"))
    }
}
