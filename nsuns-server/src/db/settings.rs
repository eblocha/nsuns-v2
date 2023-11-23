use std::time::Duration;

use config::builder::BuilderState;
use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;
use sqlx::postgres::PgConnectOptions;

use crate::settings::{CustomizeConfigBuilder, SetEnvOverride};

#[must_use]
pub fn default_timeout() -> Duration {
    Duration::from_secs(3)
}

#[must_use]
pub fn default_max_connections() -> u32 {
    5
}

#[derive(Debug, Deserialize)]
pub struct DatabaseSettings {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub password: SecretString,
    #[serde(default = "default_timeout")]
    #[serde(with = "crate::serde_duration")]
    pub timeout: Duration,
    #[serde(default = "default_max_connections")]
    pub max_connections: u32,
    pub migrations: String,
}

impl From<&DatabaseSettings> for PgConnectOptions {
    fn from(val: &DatabaseSettings) -> Self {
        PgConnectOptions::new_without_pgpass()
            .host(&val.host)
            .port(val.port)
            .database(&val.database)
            .username(&val.username)
            .password(val.password.expose_secret())
    }
}

impl<S: BuilderState> CustomizeConfigBuilder<S> for DatabaseSettings {
    fn customize_builder(
        builder: config::ConfigBuilder<S>,
        prefix: &str,
    ) -> config::ConfigBuilder<S> {
        builder
            .set_env_override_unwrap(&format!("{prefix}.host"), "DATABASE_HOST")
            .set_env_override_unwrap(&format!("{prefix}.port"), "DATABASE_PORT")
            .set_env_override_unwrap(&format!("{prefix}.username"), "DATABASE_USERNAME")
            .set_env_override_unwrap(&format!("{prefix}.password"), "DATABASE_PASSWORD")
            .set_env_override_unwrap(&format!("{prefix}.database"), "DATABASE_NAME")
            .set_env_override_unwrap(&format!("{prefix}.migrations"), "DATABASE_MIGRATIONS")
            .set_env_override_unwrap(&format!("{prefix}.timeout"), "DATABASE_CONNECTION_TIMEOUT")
            .set_env_override_unwrap(
                &format!("{prefix}.max_connections"),
                "DATABASE_MAX_CONNECTIONS",
            )
    }
}
