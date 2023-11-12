use std::{future::ready, sync::{Arc, Mutex}};

use axum::{routing::get, Router};
use metrics_exporter_prometheus::{Matcher, PrometheusBuilder};

use super::{
    names::HTTP_SERVER_REQUEST_DURATION, process::{record_process_metrics, create_system}, settings::MetricsSettings,
};

pub fn router(settings: &MetricsSettings) -> anyhow::Result<Router> {
    let recorder = PrometheusBuilder::new()
        .set_buckets_for_metric(
            Matcher::Full(HTTP_SERVER_REQUEST_DURATION.to_string()),
            &[
                0.005, 0.01, 0.025, 0.05, 0.075, 0.1, 0.25, 0.5, 0.75, 1.0, 2.5, 5.0, 7.5, 10.0,
            ],
        )?
        .install_recorder()?;

    let sys = Arc::new(Mutex::new(create_system()));

    let router = Router::new().route(
        &settings.path,
        get(move || {
            // ignore errors recording process metrics
            let _ = record_process_metrics(sys.clone());
            ready(recorder.render())
        }),
    );
    Ok(router)
}
