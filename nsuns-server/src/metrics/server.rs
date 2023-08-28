use anyhow::Context;
use axum::Router;
use hyper::Server;

use crate::{feature::Feature, server::bind, util::shutdown_signal};

use super::{
    router::router,
    settings::{MetricsFeature, MetricsSettings},
};

pub fn initialize(settings: &MetricsSettings) -> anyhow::Result<Router> {
    router(settings)
}

pub async fn run(settings: &MetricsFeature) -> anyhow::Result<()> {
    if let Feature::Enabled(settings) = settings {
        let addr = bind(settings.port)?;
        let app = initialize(settings)?;

        tracing::info!("metrics listening on {}", addr.local_addr());

        Server::builder(addr)
            .serve(app.into_make_service())
            .with_graceful_shutdown(shutdown_signal())
            .await
            .with_context(|| "metrics listener failed to start")
    } else {
        Ok(())
    }
}
