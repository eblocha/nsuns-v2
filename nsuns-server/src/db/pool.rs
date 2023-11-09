use std::path::Path;

use anyhow::Context;
use sqlx::{migrate::MigrationSource, postgres::PgPoolOptions, Acquire, Postgres};
use tracing::Instrument;

use crate::db_span;

use super::{acquire_unlogged, settings::DatabaseSettings};

pub type DB = Postgres;
pub type Pool = sqlx::Pool<DB>;

pub const DB_NAME: &str = "postgresql";

async fn run_migrations(
    migrations: impl MigrationSource<'_>,
    acquire: impl Acquire<'_, Database = DB>,
) -> anyhow::Result<()> {
    let migrator = sqlx::migrate::Migrator::new(migrations)
        .instrument(db_span!("read migrations"))
        .await
        .context("failed to read migrations")?;

    migrator
        .run(acquire)
        .instrument(db_span!("apply migrations"))
        .await
        .context("failed to perform database migrations")
}

pub async fn prepare(settings: &DatabaseSettings) -> anyhow::Result<Pool> {
    let options = settings.into();

    let pool = PgPoolOptions::new()
        .max_connections(settings.max_connections)
        .acquire_timeout(settings.timeout)
        .connect_lazy_with(options);

    let migrations = Path::new(&settings.migrations);

    let mut conn = acquire_unlogged(&pool).await?;

    run_migrations(migrations, &mut *conn).await?;

    Ok(pool)
}
