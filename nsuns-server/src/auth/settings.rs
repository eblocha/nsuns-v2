use config::builder::BuilderState;
use secrecy::SecretString;
use serde::Deserialize;

use crate::settings::{CustomizeConfigBuilder, SetEnvOverride};

#[derive(Debug, Clone, Deserialize)]
pub struct AuthSettings {
    pub jwt_encoding_secret: SecretString,
    pub jwt_decoding_secret: SecretString,
}

impl<S: BuilderState> CustomizeConfigBuilder<S> for AuthSettings {
    fn customize_builder(
        builder: config::ConfigBuilder<S>,
        prefix: &str,
    ) -> config::ConfigBuilder<S> {
        builder
            .set_env_override_unwrap(&format!("{prefix}.jwt_encoding_secret"), "JWT_SECRET")
            .set_env_override_unwrap(&format!("{prefix}.jwt_decoding_secret"), "JWT_SECRET")
            .set_env_override_unwrap(
                &format!("{prefix}.jwt_encoding_secret"),
                "JWT_ENCODING_SECRET",
            )
            .set_env_override_unwrap(
                &format!("{prefix}.jwt_decoding_secret"),
                "JWT_DECODING_SECRET",
            )
    }
}
