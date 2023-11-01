use std::{
    env::{var, VarError},
    ffi::OsStr,
};

use anyhow::{Context, Result};
use config::{builder::BuilderState, Config, ConfigBuilder, File};
use serde::Deserialize;

use crate::{
    db::settings::DatabaseSettings, feature::Feature, metrics::settings::MetricsFeature,
    openapi::settings::OpenApiFeature, tracing::settings::OpenTelemetryFeature,
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
            static_dir: Default::default(),
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
    pub opentelemetry: OpenTelemetryFeature,
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

fn bool_from_env<E>(env_var: E) -> bool
where
    E: AsRef<OsStr>,
{
    var(env_var)
        .map(|v| !v.is_empty())
        .unwrap_or_else(|err| !matches!(err, VarError::NotPresent))
}

impl Settings {
    pub fn new() -> Result<Self> {
        let config_source = var("CONFIG_SOURCE").unwrap_or_else(|_| {
            option_env!("DEFAULT_CONFIG_SOURCE")
                .unwrap_or("config/settings.toml")
                .into()
        });

        tracing::info!("reading config from {config_source}");

        let builder = Config::builder()
            .add_source(File::with_name(&config_source))
            .apply_customizations::<ServerSettings>("server")
            .apply_customizations::<DatabaseSettings>("database")
            .apply_customizations::<OpenTelemetryFeature>("opentelemetry");

        let config = builder.build();

        config
            .and_then(|cfg| cfg.try_deserialize())
            .with_context(|| format!("failed to parse settings from file: {config_source}"))
            .map(|mut settings: Settings| {
                if bool_from_env("METRICS_DISABLE") {
                    settings.metrics = Feature::Disabled;
                }

                if bool_from_env("OPENAPI_DISABLE") {
                    settings.openapi = Feature::Disabled;
                }

                if bool_from_env("OPENTELEMETRY_DISABLE") {
                    settings.opentelemetry = Feature::Disabled;
                }

                settings
            })
    }
}
