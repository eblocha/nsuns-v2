use opentelemetry_api::{propagation::TextMapPropagator, trace::SpanKind};
use opentelemetry_http::HeaderExtractor;
use opentelemetry_sdk::propagation::TraceContextPropagator;
use tower_http::{
    trace::{DefaultOnResponse, MakeSpan, OnResponse},
    LatencyUnit,
};
use tracing_opentelemetry::OpenTelemetrySpanExt;

#[derive(Debug, Clone, Copy)]
pub struct OpenTelemetryRequestSpan;

impl<B> MakeSpan<B> for OpenTelemetryRequestSpan {
    fn make_span(&mut self, request: &hyper::Request<B>) -> tracing::Span {
        let extractor = HeaderExtractor(request.headers());

        let tracing_span = tracing::info_span!(
            "HTTP request",
            http.request.method = %request.method(),
            http.version = ?request.version(),
            url.path = request.uri().path(),
            url.query = request.uri().query(),
            otel.kind = ?SpanKind::Server,
        );

        let parent = TraceContextPropagator::new().extract(&extractor);

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
