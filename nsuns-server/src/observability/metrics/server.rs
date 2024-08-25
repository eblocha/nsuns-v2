use std::net::SocketAddr;

use anyhow::Context;
use axum::Router;
use tokio::net::TcpListener;

use crate::{feature::Feature, shutdown::shutdown_signal};

use super::{
    router::router,
    settings::{MetricsFeature, MetricsSettings},
};

pub fn initialize(settings: &MetricsSettings) -> anyhow::Result<Router> {
    router(settings)
}

pub async fn run(settings: &MetricsFeature) -> anyhow::Result<()> {
    if let Feature::Enabled(settings) = settings {
        let addr = SocketAddr::from((std::net::Ipv4Addr::UNSPECIFIED, settings.port));

        let tcp = TcpListener::bind(addr).await?;

        let app = initialize(settings)?;

        tracing::info!("metrics listening on {}", addr);

        axum::serve(tcp, app.into_make_service())
            .with_graceful_shutdown(shutdown_signal())
            .await
            .context("metrics listener failed to start")
    } else {
        Ok(())
    }
}
