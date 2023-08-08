use std::{env::var, ffi::OsStr};

use anyhow::{Context, Result};
use config::{builder::BuilderState, Config, ConfigBuilder, File};
use serde::Deserialize;

use crate::{db::DatabaseSettings, feature::Feature, metrics::settings::MetricsSettings};

fn default_server_port() -> u16 {
    8080
}

#[derive(Debug, Deserialize)]
pub struct ServerSettings {
    #[serde(default = "default_server_port")]
    pub port: u16,
    #[serde(default)]
    pub static_dir: Option<String>,
}

impl Default for ServerSettings {
    fn default() -> Self {
        Self {
            port: default_server_port(),
            static_dir: Default::default(),
        }
    }
}

pub type MetricsFeature = Feature<MetricsSettings>;

impl Default for MetricsFeature {
    fn default() -> Self {
        Self::Enabled(Default::default())
    }
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    #[serde(default)]
    pub server: ServerSettings,
    #[serde(default)]
    pub metrics: MetricsFeature,
}

trait SetEnvOverride {
    fn set_env_override<K, E>(
        self,
        key: K,
        env_var: E,
    ) -> std::result::Result<Self, config::ConfigError>
    where
        Self: Sized,
        K: AsRef<str>,
        E: AsRef<OsStr>;

    /// Convenience method to unwrap a result from `set_env_override`.
    /// This is useful if the config key is static, and known to parse correctly.
    fn set_env_override_unwrap<K, E>(self, key: K, env_var: E) -> Self
    where
        Self: Sized,
        K: AsRef<str>,
        E: AsRef<OsStr>,
    {
        self.set_env_override(key, env_var).unwrap()
    }
}

impl<S: BuilderState> SetEnvOverride for ConfigBuilder<S> {
    fn set_env_override<K, E>(
        self,
        key: K,
        env_var: E,
    ) -> std::result::Result<Self, config::ConfigError>
    where
        K: AsRef<str>,
        E: AsRef<OsStr>,
    {
        self.set_override_option(key, var(env_var).ok())
    }
}

impl Settings {
    pub fn new() -> Result<Self> {
        let config_source = var("CONFIG_SOURCE").unwrap_or_else(|_| "config/settings.toml".into());

        tracing::info!("reading config from {config_source}");

        let builder = Config::builder()
            .add_source(File::with_name(&config_source))
            .set_default("server.port", 8080)
            .unwrap()
            .set_env_override_unwrap("server.port", "SERVER_PORT")
            .set_env_override_unwrap("server.static_dir", "STATIC_FILES_DIR")
            .set_env_override_unwrap("database.host", "DATABASE_HOST")
            .set_env_override_unwrap("database.port", "DATABASE_PORT")
            .set_env_override_unwrap("database.username", "DATABASE_USERNAME")
            .set_env_override_unwrap("database.password", "DATABASE_PASSWORD")
            .set_env_override_unwrap("database.migrations", "DATABASE_MIGRATIONS");

        let config = builder.build();

        config
            .and_then(|cfg| cfg.try_deserialize())
            .with_context(|| format!("failed to parse settings from file: {config_source}"))
    }
}
