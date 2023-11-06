use std::{net::SocketAddr, path::Path};

use anyhow::Context;
use axum::{extract::connect_info::Connected, Router};
use hyper::{
    server::conn::{AddrIncoming, AddrStream},
    Server,
};

use crate::{
    db::{acquire_unlogged, create_connection_pool, run_migrations},
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

    let mut conn = acquire_unlogged(&pool).await.map_err(log_error!())?;

    run_migrations(migrations, &mut *conn).await?;

    let app = router(pool, settings);
    Ok(app)
}

#[derive(Clone)]
pub struct ClientInfo {
    pub remote_addr: SocketAddr,
    pub local_addr: SocketAddr,
}

impl Connected<&AddrStream> for ClientInfo {
    fn connect_info(target: &AddrStream) -> Self {
        ClientInfo {
            remote_addr: target.remote_addr(),
            local_addr: target.local_addr(),
        }
    }
}

pub async fn run(settings: &Settings) -> anyhow::Result<()> {
    let addr = bind(settings.server.port)?;

    let app = initialize(settings).await?;

    tracing::info!("listening on {}", addr.local_addr());

    Server::builder(addr)
        .serve(app.into_make_service_with_connect_info::<ClientInfo>())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .with_context(|| "application failed to start")
}
