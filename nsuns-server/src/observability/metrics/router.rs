use std::future::ready;

use axum::{routing::get, Router};
use metrics_exporter_prometheus::{Matcher, PrometheusBuilder};
use metrics_process::Collector;

use super::settings::MetricsSettings;

pub fn router(settings: &MetricsSettings) -> anyhow::Result<Router> {
    let recorder = PrometheusBuilder::new()
        .set_buckets_for_metric(
            Matcher::Full("http.server.request.duration".to_owned()),
            &[
                0.005, 0.01, 0.025, 0.05, 0.075, 0.1, 0.25, 0.5, 0.75, 1.0, 2.5, 5.0, 7.5, 10.0,
            ],
        )?
        .install_recorder()?;

    let process_collector = Collector::default();
    process_collector.describe();

    let router = Router::new().route(
        &settings.path,
        get(move || {
            process_collector.collect();
            ready(recorder.render())
        }),
    );
    Ok(router)
}
