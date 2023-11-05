use std::path::Path;

use anyhow::Context;
use axum::Router;
use hyper::{server::conn::AddrIncoming, Server};

use crate::{
    db::{acquire_anyhow, create_connection_pool, run_migrations},
    log_error,
    router::router,
    settings::Settings,
    shutdown::shutdown_signal,
};

pub fn bind(port: u16) -> anyhow::Result<AddrIncoming> {
    let addr = std::net::SocketAddr::from((std::net::Ipv4Addr::UNSPECIFIED, port));

    AddrIncoming::bind(&addr).map_err(Into::into)
}

#[tracing::instrument(skip_all)]
pub async fn initialize(settings: &Settings) -> anyhow::Result<Router> {
    tracing::debug!("loaded configuration:\n{:#?}", settings);

    let pool = create_connection_pool(&settings.database);

    let migrations = Path::new(&settings.database.migrations);

    let mut conn = acquire_anyhow(&pool).await.map_err(log_error!())?;
    run_migrations(migrations, &mut conn).await?;

    let app = router(pool, settings);
    Ok(app)
}

pub async fn run(settings: &Settings) -> anyhow::Result<()> {
    let addr = bind(settings.server.port)?;

    let app = initialize(settings).await?;

    tracing::info!("listening on {}", addr.local_addr());

    Server::builder(addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .with_context(|| "application failed to start")
}
