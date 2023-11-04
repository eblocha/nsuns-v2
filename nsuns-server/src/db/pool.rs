use anyhow::Context;
use sqlx::{
    migrate::MigrationSource,
    postgres::{PgConnectOptions, PgPoolOptions},
    Acquire, Postgres
};
use tracing::Instrument;

use crate::db_span;

use super::settings::DatabaseSettings;

pub type DB = Postgres;
pub type Pool = sqlx::Pool<DB>;

pub const DB_NAME: &str = "postgresql";

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
        .instrument(db_span!("apply migrations"))
        .await
        .with_context(|| "failed to perform database migrations")
}
