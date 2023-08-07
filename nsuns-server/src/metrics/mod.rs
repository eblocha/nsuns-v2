use anyhow::{Context, Result};
use axum::Router;

use crate::util::shutdown_signal;

use self::{router::router, settings::MetricsSettings};

pub mod middleware;
pub mod router;
pub mod settings;

pub fn initialize_metrics_server(settings: &MetricsSettings) -> Result<Router> {
    return router(settings);
}

pub async fn metrics_server(settings: &MetricsSettings) -> Result<()> {
    let app = initialize_metrics_server(settings)?;
    let addr = std::net::SocketAddr::from((std::net::Ipv4Addr::UNSPECIFIED, settings.port));

    tracing::info!("metrics listening on {addr}");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .with_context(|| "metrics listener failed to start")
}
