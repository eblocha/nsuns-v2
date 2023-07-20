use std::path::Path;

use anyhow::{Context, Result};
use axum::Router;
use db::run_migrations;
use router::router;
use settings::Settings;

use crate::util::shutdown_signal;

pub mod db;
pub mod error;
pub mod movements;
pub mod program;
pub mod router;
pub mod sets;
pub mod settings;
pub mod profiles;
pub mod util;
pub mod validation;

pub async fn initialize_api_server(settings: &Settings) -> Result<Router> {
    let pool = db::create_connection_pool(&settings.database)
        .with_context(|| "failed to create connection pool")?;

    let migrations = Path::new(&settings.database.migrations);

    run_migrations(migrations, &pool).await?;

    let app = router(pool, settings)?;
    Ok(app)
}

pub async fn api_server(settings: &Settings) -> Result<()> {
    let app = initialize_api_server(settings).await?;
    let addr = std::net::SocketAddr::from((std::net::Ipv4Addr::UNSPECIFIED, settings.server.port));

    tracing::info!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .with_context(|| "application failed to start")
}
