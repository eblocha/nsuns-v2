use config::builder::BuilderState;
use serde::Deserialize;

use crate::{
    feature::{Feature, ENABLED_KEY},
    settings::{CustomizeConfigBuilder, SetEnvOverride},
};

fn default_swagger_path() -> String {
    "/swagger-ui".to_string()
}

fn default_openapi_path() -> String {
    "/api-docs/openapi.json".to_string()
}

#[derive(Debug, Deserialize)]
pub struct OpenApiSettings {
    #[serde(default = "default_swagger_path")]
    pub swagger_path: String,
    #[serde(default = "default_openapi_path")]
    pub openapi_path: String,
}

impl Default for OpenApiSettings {
    fn default() -> Self {
        Self {
            swagger_path: default_swagger_path(),
            openapi_path: default_openapi_path(),
        }
    }
}

pub type OpenApiFeature = Feature<OpenApiSettings>;

impl Default for OpenApiFeature {
    fn default() -> Self {
        Self::Disabled
    }
}

impl<S: BuilderState> CustomizeConfigBuilder<S> for OpenApiSettings {
    fn customize_builder(
        builder: config::ConfigBuilder<S>,
        prefix: &str,
    ) -> config::ConfigBuilder<S> {
        let builder = builder
            .set_env_override_unwrap(&format!("{prefix}.{ENABLED_KEY}"), "OPENAPI_ENABLED")
            .set_env_override_unwrap(&format!("{prefix}.swagger_path"), "SWAGGER_UI_PATH")
            .set_env_override_unwrap(&format!("{prefix}.openapi_path"), "OPENAPI_PATH");

        #[cfg(not(feature = "openapi"))]
        let builder = builder
            .set_override(format!("{prefix}.{ENABLED_KEY}"), false)
            .unwrap();

        builder
    }
}
