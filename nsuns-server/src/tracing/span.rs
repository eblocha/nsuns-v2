use http::Version;
use opentelemetry_api::{propagation::TextMapPropagator, trace::SpanKind};
use opentelemetry_http::{HeaderExtractor, HeaderInjector};
use opentelemetry_sdk::propagation::TraceContextPropagator;
use opentelemetry_semantic_conventions as semcov;
use tower_http::{
    trace::{DefaultOnResponse, MakeSpan, OnResponse},
    LatencyUnit,
};
use tracing::{field::Empty, Span};
use tracing_opentelemetry::OpenTelemetrySpanExt;
use uuid::Uuid;

/// Set the request span's parent as the incoming otel span, if present.
#[derive(Debug, Clone)]
pub struct OpenTelemetryRequestSpan;

impl<B> MakeSpan<B> for OpenTelemetryRequestSpan {
    fn make_span(&mut self, request: &http::Request<B>) -> tracing::Span {
        let extractor = HeaderExtractor(request.headers());

        let parent = TraceContextPropagator::new().extract(&extractor);

        let request_id = Uuid::new_v4();

        let tracing_span = tracing::info_span!(
            "HTTP request",
            otel.kind = ?SpanKind::Server,
            trace_id = Empty,
            request_id = %request_id,
        );

        tracing_span.set_parent(parent);

        update_span_from_request(&tracing_span, request);

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
        response: &http::Response<B>,
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

/// Update the given span to record fields from the http request
pub fn update_span_from_request<B>(span: &tracing::Span, request: &http::Request<B>) {
    span.record(
        semcov::trace::HTTP_REQUEST_METHOD.as_str(),
        request.method().as_str(),
    );
    span.record(
        semcov::trace::USER_AGENT_ORIGINAL.as_str(),
        user_agent(request),
    );
    span.record(
        semcov::trace::HTTP_RESPONSE_BODY_SIZE.as_str(),
        response_body_size(request),
    );
    span.record(semcov::trace::NETWORK_PROTOCOL_NAME.as_str(), "http");
    span.record(
        semcov::trace::NETWORK_PROTOCOL_VERSION.as_str(),
        protocol_version(request),
    );
    span.record(semcov::trace::SERVER_ADDRESS.as_str(), http_host(request));
    span.record(semcov::trace::URL_PATH.as_str(), request.uri().path());
    span.record(semcov::trace::URL_QUERY.as_str(), request.uri().query());
    span.record(
        semcov::trace::URL_SCHEME.as_str(),
        request.uri().scheme_str().unwrap_or_default(),
    );
}

/// Update the given span to record fields from the http response
pub fn update_span_from_response<B>(span: &tracing::Span, response: &http::Response<B>) {
    span.record(
        semcov::trace::HTTP_RESPONSE_STATUS_CODE.as_str(),
        response.status().as_u16(),
    );

    if response.status().is_server_error() || response.status().is_client_error() {
        span.record(semcov::trace::OTEL_STATUS_CODE.as_str(), "ERROR");
    } else {
        span.record(semcov::trace::OTEL_STATUS_CODE.as_str(), "OK");
    }

    span.record(
        semcov::trace::OTEL_STATUS_DESCRIPTION.as_str(),
        response.status().canonical_reason(),
    );
}

#[inline]
fn user_agent<B>(req: &http::Request<B>) -> &str {
    req.headers()
        .get(http::header::USER_AGENT)
        .map_or("", |h| h.to_str().unwrap_or_default())
}

#[inline]
fn response_body_size<B>(req: &http::Request<B>) -> &str {
    req.headers()
        .get(http::header::CONTENT_LENGTH)
        .map_or("", |h| h.to_str().unwrap_or_default())
}

#[inline]
fn protocol_version<B>(req: &http::Request<B>) -> &str {
    match req.version() {
        Version::HTTP_09 => "0.9",
        Version::HTTP_10 => "1.0",
        Version::HTTP_11 => "1.1",
        Version::HTTP_2 => "2.0",
        Version::HTTP_3 => "3.0",
        _ => "",
    }
}

#[inline]
fn http_host<B>(req: &http::Request<B>) -> &str {
    req.headers()
        .get(http::header::HOST)
        .map_or(req.uri().host(), |h| h.to_str().ok())
        .unwrap_or_default()
}

pub trait WithSpan: Sized {
    fn with_current_span(self) -> Self {
        self.with_span(&Span::current())
    }

    fn with_span(self, span: &Span) -> Self;
}

impl<B> WithSpan for http::Request<B> {
    fn with_span(mut self, span: &Span) -> Self {
        update_span_from_request(span, &self);

        TraceContextPropagator::new()
            .inject_context(&span.context(), &mut HeaderInjector(self.headers_mut()));

        self
    }
}

impl<B> WithSpan for http::Response<B> {
    fn with_span(mut self, span: &Span) -> Self {
        update_span_from_response(&span, &self);

        TraceContextPropagator::new()
            .inject_context(&span.context(), &mut HeaderInjector(self.headers_mut()));

        self
    }
}
