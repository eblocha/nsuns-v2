use std::time::Instant;

use axum::{
    http::Request,
    middleware::{from_fn, Next},
    response::IntoResponse,
    Router,
};
use http::Response;

use crate::{
    metrics::names::{HTTP_SERVER_ACTIVE_REQUESTS, HTTP_SERVER_REQUEST_DURATION},
    observability::attributes::HttpRequestAttributes,
};

use super::settings::MetricsFeature;

type Attribute = (&'static str, String);

fn build_attrs(attrs: &HttpRequestAttributes) -> Vec<Attribute> {
    let mut duration_attrs = Vec::with_capacity(9);

    duration_attrs.extend([
        ("otel.scope.name", "nsuns-server".to_string()),
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

fn finalize_attrs<B>(attrs: &mut Vec<Attribute>, response: &Response<B>) {
    attrs.push((
        "http.response.status_code",
        response.status().as_u16().to_string(),
    ));
}

async fn track_metrics<B>(req: Request<B>, next: Next<B>) -> impl IntoResponse {
    let start = Instant::now();

    let attrs: HttpRequestAttributes = (&req).into();

    let method = attrs.http_request_method.clone();

    let active_requests_attrs = [
        ("otel.scope.name", "nsuns-server".to_string()),
        ("http.request.method", method.to_string()),
    ];

    let mut duration_attrs = build_attrs(&attrs);

    metrics::increment_gauge!(HTTP_SERVER_ACTIVE_REQUESTS, 1.0, &active_requests_attrs);

    let response = next.run(req).await;

    finalize_attrs(&mut duration_attrs, &response);

    metrics::decrement_gauge!(HTTP_SERVER_ACTIVE_REQUESTS, 1.0, &active_requests_attrs);
    metrics::histogram!(
        HTTP_SERVER_REQUEST_DURATION,
        start.elapsed(),
        &duration_attrs
    );

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
