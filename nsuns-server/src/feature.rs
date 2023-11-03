use config::{builder::BuilderState, ConfigBuilder};
use serde::{de, Deserialize, Deserializer};
use serde_json::{Map, Value};

use crate::settings::CustomizeConfigBuilder;

#[derive(Debug)]
pub enum Feature<T> {
    Enabled(T),
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
        T::customize_builder(builder, prefix)
    }
}

impl<'de, T> Deserialize<'de> for Feature<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Feature<T>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut map = Map::deserialize(deserializer)?;

        let enabled = map
            .remove("enabled")
            .ok_or_else(|| de::Error::missing_field("enabled"))
            .map(Deserialize::deserialize)?
            .map_err(de::Error::custom)?;
        let rest = Value::Object(map);

        if enabled {
            T::deserialize(rest)
                .map(Feature::Enabled)
                .map_err(de::Error::custom)
        } else {
            Ok(Feature::Disabled)
        }
    }
}
