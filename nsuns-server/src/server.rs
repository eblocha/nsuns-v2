use std::net::SocketAddr;

use anyhow::Context;
use axum::{extract::connect_info::Connected, serve::IncomingStream, Router};
use tokio::net::TcpListener;

use crate::{
    db, log_error,
    router::{router, AppState},
    settings::Settings,
    shutdown::shutdown_signal,
};

#[tracing::instrument(skip_all)]
pub async fn initialize(settings: &Settings) -> anyhow::Result<Router> {
    tracing::debug!("loaded configuration:\n{:#?}", settings);

    let pool = db::prepare(&settings.database)
        .await
        .map_err(log_error!())?;

    router(
        AppState {
            pool,
            keys: (&settings.auth).into(),
        },
        settings,
    )
}

#[derive(Clone)]
pub struct ClientInfo {
    pub remote_addr: SocketAddr,
    pub local_addr: Option<SocketAddr>,
}

impl Connected<IncomingStream<'_>> for ClientInfo {
    fn connect_info(target: IncomingStream<'_>) -> Self {
        ClientInfo {
            remote_addr: target.remote_addr(),
            local_addr: target.local_addr().ok()
        }
    }
}

pub async fn run(settings: &Settings) -> anyhow::Result<()> {
    let addr = SocketAddr::from((std::net::Ipv4Addr::UNSPECIFIED, settings.server.port));
    let tcp = TcpListener::bind(addr).await?;

    let app = initialize(settings).await?;

    tracing::info!("listening on {}", addr);

    axum::serve(tcp, app.into_make_service_with_connect_info::<ClientInfo>())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("application failed to start")
}
