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

pub const ENABLED_KEY: &str = "enabled";

fn parse_failed(v: &Value) -> String {
    format!(r#"failed to parse as boolean: {v:#}"#)
}

fn try_parse_str(s: &str) -> Option<bool> {
    if let Ok(b) = s.parse() {
        return Some(b);
    }

    let lowercase = s.to_lowercase();

    match lowercase.as_str() {
        "yes" => Some(true),
        "no" => Some(false),
        "y" => Some(true),
        "n" => Some(false),
        "t" => Some(true),
        "f" => Some(false),
        "1" => Some(true),
        "0" => Some(false),
        _ => None,
    }
}

fn coerce_bool(v: Value) -> Result<bool, String> {
    if let Some(b) = v.as_bool() {
        return Ok(b);
    }

    if let Some(s) = v.as_str() {
        return try_parse_str(s).ok_or_else(|| parse_failed(&v));
    }

    if let Some(n) = v.as_i64() {
        return match n {
            1 => Ok(true),
            0 => Ok(false),
            _ => Err(parse_failed(&v)),
        };
    }

    if let Some(n) = v.as_u64() {
        return match n {
            1 => Ok(true),
            0 => Ok(false),
            _ => Err(parse_failed(&v)),
        };
    }

    Err(parse_failed(&v))
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
            .remove(ENABLED_KEY)
            .ok_or_else(|| de::Error::missing_field(ENABLED_KEY))
            .map(coerce_bool)?
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
