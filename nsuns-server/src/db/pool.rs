use std::{fmt::Debug, time::Duration};

use anyhow::Context;
use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;
use sqlx::{
    migrate::MigrationSource,
    postgres::{PgConnectOptions, PgPoolOptions},
    Acquire, Postgres,
};
use tracing::Instrument;

pub type DB = Postgres;
pub type Pool = sqlx::Pool<DB>;

pub fn default_timeout() -> Duration {
    Duration::from_secs(3)
}

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

pub fn create_connection_pool(settings: &DatabaseSettings) -> Pool {
    let options: PgConnectOptions = settings.into();

    PgPoolOptions::new()
        .max_connections(settings.max_connections)
        .acquire_timeout(settings.timeout)
        .connect_lazy_with(options)
}

pub async fn run_migrations(
    migrations: impl MigrationSource<'_>,
    acquire: impl Acquire<'_, Database = DB>,
) -> anyhow::Result<()> {
    let migrator = sqlx::migrate::Migrator::new(migrations)
        .await
        .with_context(|| "failed to read migrations")?;

    migrator
        .run(acquire)
        .instrument(tracing::info_span!("apply migrations"))
        .await
        .with_context(|| "failed to perform database migrations")
}
