use std::time::Instant;

use axum::{
    extract::MatchedPath,
    http::Request,
    middleware::{from_fn, Next},
    response::IntoResponse,
    Router,
};

use super::settings::MetricsFeature;

async fn track_metrics<B>(req: Request<B>, next: Next<B>) -> impl IntoResponse {
    let start = Instant::now();
    let path = if let Some(matched_path) = req.extensions().get::<MatchedPath>() {
        matched_path.as_str().to_owned()
    } else {
        req.uri().path().to_owned()
    };
    let method = req.method().clone();

    let response = next.run(req).await;

    let latency = start.elapsed().as_secs_f64();
    let status = response.status().as_u16().to_string();

    let labels = [
        ("method", method.to_string()),
        ("path", path),
        ("status", status),
    ];

    metrics::increment_counter!("http_requests_total", &labels);
    metrics::histogram!("http_requests_duration_seconds", latency, &labels);

    response
}

pub trait WithMetrics {
    fn with_metrics(self, settings: &MetricsFeature) -> Self;
}

impl<S> WithMetrics for Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    fn with_metrics(self, settings: &MetricsFeature) -> Self {
        if settings.is_enabled() {
            self.route_layer(from_fn(track_metrics))
        } else {
            self
        }
    }
}
