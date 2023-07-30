use std::{fmt::Debug, path::Path, time::Duration};

use anyhow::Context;
use serde::Deserialize;
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    Acquire, Postgres, Transaction,
};

use crate::error::{ErrorWithStatus, OperationResult};

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

pub fn create_connection_pool(settings: &DatabaseSettings) -> Pool {
    let options: PgConnectOptions = settings.into();

    PgPoolOptions::new()
        .max_connections(settings.max_connections)
        .acquire_timeout(settings.timeout)
        .connect_lazy_with(options)
}

pub async fn run_migrations<'a>(
    migrations: &Path,
    acquire: impl Acquire<'a, Database = DB>,
) -> anyhow::Result<()> {
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
) -> OperationResult<Transaction<'a, DB>> {
    acquire
        .begin()
        .await
        .with_context(|| "failed to start a transaction")
        .map_err(Into::into)
}

/// Commit the transaction if the result is Ok, otherwise rollback.
/// This may transform Ok into Err if the commit fails.
#[inline]
pub async fn commit_ok<T, E>(result: Result<T, E>, tx: Transaction<'_, DB>) -> OperationResult<T>
where
    E: Into<ErrorWithStatus<anyhow::Error>> + Debug,
{
    match result {
        Ok(_) => {
            tx.commit()
                .await
                .with_context(|| "failed to commit transaction")?;
        }
        Err(ref e) => {
            tx.rollback()
                .await
                .with_context(|| format!("failed to rollback transaction caused by: {e:?}"))?;
        }
    };

    result.map_err(Into::into)
}
