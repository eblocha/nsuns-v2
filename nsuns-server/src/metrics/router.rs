use std::future::ready;

use anyhow::Result;
use axum::{routing::get, Router};
use metrics_exporter_prometheus::PrometheusBuilder;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::Level;

use super::settings::MetricsSettings;

pub fn router(settings: &MetricsSettings) -> Result<Router> {
    let recorder = PrometheusBuilder::new().install_recorder()?;
    let router = Router::new()
        .route(&settings.path, get(move || ready(recorder.render())))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        );
    Ok(router)
}
