use anyhow::Context;
use sqlx::{migrate::MigrationSource, Acquire, Database, Postgres, postgres::PgConnectOptions};
use tracing::Instrument;

use crate::db_span;

use super::{settings::DatabaseSettings, unpooled::UnPooled};

pub type DB = Postgres;
pub type Connection = <DB as Database>::Connection;
pub type Pool = UnPooled;
pub type PoolConnection = sqlx::pool::PoolConnection<DB>;

pub const DB_NAME: &str = "postgresql";

pub fn create_connection_pool(settings: &DatabaseSettings) -> Pool {
    let options: PgConnectOptions = settings.into();
    options.into()

    // PgPoolOptions::new()
    //     .max_connections(settings.max_connections)
    //     .acquire_timeout(settings.timeout)
    // .connect_lazy_with(options)
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
