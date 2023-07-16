use std::{path::Path, time::Duration};

use anyhow::{Context, Result};
use serde::Deserialize;
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    Acquire, Postgres, Transaction,
};

use crate::error::{Error, IntoResult};

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
    pub password: String,
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
            .password(&val.password)
    }
}

pub fn create_connection_pool(settings: &DatabaseSettings) -> Result<Pool> {
    let options: PgConnectOptions = settings.into();

    let pool = PgPoolOptions::new()
        .max_connections(settings.max_connections)
        .acquire_timeout(settings.timeout)
        .connect_lazy_with(options);

    Ok(pool)
}

pub async fn run_migrations<'a>(
    migrations: &Path,
    acquire: impl Acquire<'a, Database = DB>,
) -> Result<()> {
    let migrator = sqlx::migrate::Migrator::new(migrations)
        .await
        .with_context(|| "failed to read migrations")?;

    migrator
        .run(acquire)
        .await
        .with_context(|| "failed to perform database migrations")
}

/// Acquire a new transaction
#[inline]
pub async fn transaction<'a>(
    acquire: impl Acquire<'a, Database = DB>,
) -> crate::error::Result<Transaction<'a, DB>> {
    acquire
        .begin()
        .await
        .with_context(|| "failed to start a transaction")
        .into_result()
}

/// Commit the transaction if the result is Ok, otherwise rollback.
/// This may transform Ok into Err if the commit fails.
#[inline]
pub async fn commit_ok<T, E>(
    result: core::result::Result<T, E>,
    tx: Transaction<'_, DB>,
) -> crate::error::Result<T>
where
    E: Into<Error>,
{
    if result.is_ok() {
        tx.commit().await?;
    } else {
        tx.rollback().await?;
    }

    result.into_result()
}
