use std::{env::var, ffi::OsStr};

use anyhow::{Context, Result};
use config::{
    builder::{BuilderState, DefaultState},
    Config, ConfigBuilder, File,
};
use serde::Deserialize;

use crate::{
    auth::settings::AuthSettings,
    db::settings::DatabaseSettings,
    observability::{metrics::settings::MetricsFeature, tracing::settings::LogSettings},
    openapi::settings::OpenApiFeature,
};

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
            static_dir: Option::default(),
        }
    }
}

impl<S: BuilderState> CustomizeConfigBuilder<S> for ServerSettings {
    fn customize_builder(builder: ConfigBuilder<S>, prefix: &str) -> ConfigBuilder<S> {
        builder
            .set_env_override_unwrap(&format!("{prefix}.port"), "SERVER_PORT")
            .set_env_override_unwrap(&format!("{prefix}.static_dir"), "STATIC_FILES_DIR")
    }
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    #[serde(default)]
    pub server: ServerSettings,
    #[serde(default)]
    pub metrics: MetricsFeature,
    #[serde(default)]
    pub openapi: OpenApiFeature,
    #[serde(default)]
    pub logging: LogSettings,
    pub auth: AuthSettings,
}

pub trait SetEnvOverride {
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
    #[must_use]
    fn set_env_override_unwrap<E>(self, key: &str, env_var: E) -> Self
    where
        Self: Sized,
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

/// Customize the config builder. Useful for setting overrides to apply from env vars.
pub trait CustomizeConfigBuilder<S: BuilderState> {
    fn customize_builder(builder: ConfigBuilder<S>, prefix: &str) -> ConfigBuilder<S>;
}

trait ApplyCustomizations<S: BuilderState> {
    fn apply_customizations<C: CustomizeConfigBuilder<S>>(self, prefix: &str) -> Self;
}

impl<S: BuilderState> ApplyCustomizations<S> for ConfigBuilder<S> {
    fn apply_customizations<C: CustomizeConfigBuilder<S>>(self, prefix: &str) -> Self {
        C::customize_builder(self, prefix)
    }
}

#[derive(Debug)]
struct FileSourceList(String);

impl FileSourceList {
    pub fn configure(&self, builder: ConfigBuilder<DefaultState>) -> ConfigBuilder<DefaultState> {
        self.0
            .split(',')
            .filter_map(|config_file_name| {
                let filename = config_file_name.trim();
                if filename.is_empty() {
                    None
                } else {
                    Some(filename)
                }
            })
            .fold(builder, |builder, file| {
                builder.add_source(File::with_name(file))
            })
    }
}

impl Settings {
    pub fn new() -> Result<Self> {
        let config_source = FileSourceList(var("CONFIG_SOURCE").unwrap_or_else(|_| {
            option_env!("DEFAULT_CONFIG_SOURCE")
                .unwrap_or("config/settings.toml")
                .into()
        }));

        let builder = Config::builder();

        let builder = config_source
            .configure(builder)
            .apply_customizations::<ServerSettings>("server")
            .apply_customizations::<DatabaseSettings>("database")
            .apply_customizations::<LogSettings>("logging")
            .apply_customizations::<OpenApiFeature>("openapi")
            .apply_customizations::<MetricsFeature>("metrics");

        let config = builder.build();

        config
            .and_then(config::Config::try_deserialize)
            .with_context(|| format!("failed to parse settings! config_source={config_source:?}"))
    }
}
