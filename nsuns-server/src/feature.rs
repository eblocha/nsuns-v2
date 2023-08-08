use serde::Deserialize;

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
