use opentelemetry_api::{propagation::TextMapPropagator, trace::SpanKind};
use opentelemetry_http::HeaderExtractor;
use opentelemetry_sdk::propagation::TraceContextPropagator;
use tower_http::{
    trace::{DefaultOnResponse, MakeSpan, OnResponse},
    LatencyUnit,
};
use tracing::field::Empty;
use tracing_opentelemetry::OpenTelemetrySpanExt;
use uuid::Uuid;

/// Set the request span's parent as the incoming otel span, if present.
#[derive(Debug, Clone)]
pub struct OpenTelemetryRequestSpan;

impl<B> MakeSpan<B> for OpenTelemetryRequestSpan {
    fn make_span(&mut self, request: &hyper::Request<B>) -> tracing::Span {
        let extractor = HeaderExtractor(request.headers());

        let parent = TraceContextPropagator::new().extract(&extractor);

        let request_id = Uuid::new_v4();

        let tracing_span = tracing::info_span!(
            "HTTP request",
            http.request.method = %request.method(),
            http.response.status_code = Empty,
            http.user_agent = user_agent(request),
            network.protocol.version = ?request.version(),
            server.host = http_host(request),
            url.path = request.uri().path(),
            url.query = request.uri().query(),
            url.scheme = request_scheme(request),
            otel.name = %request.method(),
            otel.kind = ?SpanKind::Server,
            trace_id = Empty,
            request_id = %request_id,
        );

        tracing_span.set_parent(parent);

        tracing_span
    }
}

/// Record the request latency with dynamic units.
///
/// If the latency is under 1ms, it will be reported in μs.
/// Otherwise, it will be reported in ms.
#[derive(Debug, Clone)]
pub struct DynamicLatencyUnitOnResponse(pub DefaultOnResponse);

impl<B> OnResponse<B> for DynamicLatencyUnitOnResponse {
    fn on_response(
        self,
        response: &hyper::Response<B>,
        latency: std::time::Duration,
        span: &tracing::Span,
    ) {
        let unit = if latency.as_secs() > 0 || latency.subsec_nanos() > 1_000_000 {
            LatencyUnit::Millis
        } else {
            LatencyUnit::Micros
        };

        self.0
            .latency_unit(unit)
            .on_response(response, latency, span)
    }
}

#[inline]
fn http_host<B>(req: &hyper::Request<B>) -> &str {
    req.headers()
        .get(hyper::header::HOST)
        .map_or(req.uri().host(), |h| h.to_str().ok())
        .unwrap_or_default()
}

#[inline]
fn request_scheme<B>(req: &hyper::Request<B>) -> &str {
    req.uri().scheme_str().unwrap_or_default()
}

#[inline]
fn user_agent<B>(req: &hyper::Request<B>) -> &str {
    req.headers()
        .get(hyper::header::USER_AGENT)
        .map_or("", |h| h.to_str().unwrap_or_default())
}