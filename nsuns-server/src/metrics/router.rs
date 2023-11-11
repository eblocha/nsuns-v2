use std::future::ready;

use axum::{routing::get, Router};
use metrics_exporter_prometheus::PrometheusBuilder;

use crate::observability::tracing::middleware::WithTracing;

use super::settings::MetricsSettings;

pub fn router(settings: &MetricsSettings) -> anyhow::Result<Router> {
    let recorder = PrometheusBuilder::new().install_recorder()?;
    let router = Router::new()
        .route(&settings.path, get(move || ready(recorder.render())))
        .with_tracing();
    Ok(router)
}
