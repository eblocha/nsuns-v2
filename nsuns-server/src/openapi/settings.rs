use serde::Deserialize;

use crate::feature::Feature;

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
