use std::time::Instant;

use axum::{
    extract::Request,
    middleware::{from_fn, Next},
    response::{IntoResponse, Response},
    Router,
};

use crate::observability::attributes::HttpRequestAttributes;

use super::settings::MetricsFeature;

type Attribute = (&'static str, String);

fn build_attrs(attrs: &HttpRequestAttributes) -> Vec<Attribute> {
    let mut duration_attrs = Vec::with_capacity(6);

    duration_attrs.extend([
        ("http.request.method", attrs.http_request_method.to_string()),
        (
            "network.protocol.name",
            attrs.network_protocol_name.to_string(),
        ),
    ]);

    if let Some(protocol_version) = attrs.network_protocol_version {
        duration_attrs.push(("network.protocol.name", protocol_version.to_string()));
    }

    if let Some(route) = attrs.http_route {
        duration_attrs.push(("http.route", route.to_string()));
    }

    if let Some(url_scheme) = attrs.url_scheme {
        duration_attrs.push(("url.scheme", url_scheme.to_string()));
    }

    duration_attrs
}

fn finalize_attrs(attrs: &mut Vec<Attribute>, response: &Response) {
    attrs.push((
        "http.response.status_code",
        response.status().as_u16().to_string(),
    ));
}

async fn track_metrics(req: Request, next: Next) -> impl IntoResponse {
    let start = Instant::now();

    let attrs: HttpRequestAttributes = (&req).into();

    let method = attrs.http_request_method.clone();

    let active_requests_attrs = [("http.request.method", method.to_string())];

    let mut duration_attrs = build_attrs(&attrs);

    metrics::increment_gauge!("http.server.active_requests", 1.0, &active_requests_attrs);

    let response = next.run(req).await;

    finalize_attrs(&mut duration_attrs, &response);

    metrics::decrement_gauge!("http.server.active_requests", 1.0, &active_requests_attrs);
    metrics::histogram!(
        "http.server.request.duration",
        start.elapsed(),
        &duration_attrs
    );

    response
}

pub trait WithMetrics {
    #[must_use]
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
