use std::{fmt, time::Duration};

use http::Version;
use opentelemetry_api::{
    propagation::TextMapPropagator,
    trace::{SpanKind, TraceContextExt},
};
use opentelemetry_http::{HeaderExtractor, HeaderInjector};
use opentelemetry_sdk::propagation::TraceContextPropagator;
use opentelemetry_semantic_conventions as semcov;
use tower_http::{
    trace::{MakeSpan, OnResponse},
    LatencyUnit,
};
use tracing::{field::Empty, Span};
use tracing_opentelemetry::OpenTelemetrySpanExt;

/// Set the request span's parent as the incoming otel span, if present.
#[derive(Debug, Clone)]
pub struct OpenTelemetryRequestSpan;

impl<B> MakeSpan<B> for OpenTelemetryRequestSpan {
    fn make_span(&mut self, request: &http::Request<B>) -> tracing::Span {
        let extractor = HeaderExtractor(request.headers());

        let parent = TraceContextPropagator::new().extract(&extractor);

        let tracing_span = tracing::info_span!(
            "HTTP request",
            otel.kind = ?SpanKind::Server,
            trace_id = Empty,
            http.request.method = request.method().as_str(),
            user_agent.original = user_agent(request),
            network.protocol.name = "http",
            network.protocol.version = protocol_version(request),
            server.address = http_host(request),
            url.path = request.uri().path(),
            url.query = request.uri().query(),
            url.scheme = request.uri().scheme_str().unwrap_or_default(),
            // set in response
            http.response.status_code = Empty,
            otel.status_code = Empty,
            otel.status_description = Empty,
            http.response.body.size = Empty,
        );

        tracing_span.set_parent(parent);

        tracing_span
    }
}

struct Latency {
    unit: LatencyUnit,
    duration: Duration,
}

impl fmt::Display for Latency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.unit {
            LatencyUnit::Millis => write!(f, "{} ms", self.duration.as_millis()),
            LatencyUnit::Micros => write!(f, "{} μs", self.duration.as_micros()),
            LatencyUnit::Nanos => write!(f, "{} ns", self.duration.as_nanos()),
            _ => write!(f, "{} s", self.duration.as_secs_f64()),
        }
    }
}

/// Updates the request span with response information and logs a request event.
#[derive(Debug, Clone)]
pub struct UpdateSpanOnResponse;

impl<B> OnResponse<B> for UpdateSpanOnResponse {
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

        update_span_from_response(span, response);

        let latency = Latency {
            unit,
            duration: latency,
        };

        tracing::info!(
            %latency,
            "finished processing request"
        );
    }
}

/// Update the given span to record fields from the http response
pub fn update_span_from_response<B>(span: &tracing::Span, response: &http::Response<B>) {
    span.record(
        semcov::trace::HTTP_RESPONSE_STATUS_CODE.as_str(),
        response.status().as_u16(),
    );

    if response.status().is_server_error() {
        span.record(semcov::trace::OTEL_STATUS_CODE.as_str(), "ERROR");
    }

    span.record(
        semcov::trace::HTTP_RESPONSE_BODY_SIZE.as_str(),
        response_body_size(response),
    );

    span.record("trace_id", current_trace_id());
}

pub fn current_trace_id() -> Option<String> {
    let cx = Span::current().context();
    let s = cx.span();
    let span_context = s.span_context();

    span_context
        .is_valid()
        .then(|| span_context.trace_id().to_string())
}

#[inline]
fn user_agent<B>(req: &http::Request<B>) -> Option<&str> {
    req.headers()
        .get(http::header::USER_AGENT)
        .map_or(None, |h| h.to_str().ok())
}

#[inline]
fn response_body_size<B>(res: &http::Response<B>) -> Option<u64> {
    res.headers()
        .get(http::header::CONTENT_LENGTH)
        .map_or(None, |h| {
            h.to_str()
                .unwrap_or_default()
                .parse()
                .map(Some)
                .unwrap_or_default()
        })
}

#[inline]
fn protocol_version<B>(req: &http::Request<B>) -> Option<&str> {
    match req.version() {
        Version::HTTP_09 => Some("0.9"),
        Version::HTTP_10 => Some("1.0"),
        Version::HTTP_11 => Some("1.1"),
        Version::HTTP_2 => Some("2.0"),
        Version::HTTP_3 => Some("3.0"),
        _ => None,
    }
}

#[inline]
fn http_host<B>(req: &http::Request<B>) -> Option<&str> {
    req.headers()
        .get(http::header::HOST)
        .map_or(req.uri().host(), |h| h.to_str().ok())
}

pub trait WithSpan: Sized {
    fn with_current_span(self) -> Self {
        self.with_span(&Span::current())
    }

    fn with_span(self, span: &Span) -> Self;
}

impl<B> WithSpan for http::Request<B> {
    /// Update outbound requests to contain otel tracing headers for the current span
    fn with_span(mut self, span: &Span) -> Self {
        TraceContextPropagator::new()
            .inject_context(&span.context(), &mut HeaderInjector(self.headers_mut()));

        self
    }
}

impl<B> WithSpan for http::Response<B> {
    /// Update outbound responses to contain otel tracing headers for the current span
    fn with_span(mut self, span: &Span) -> Self {
        TraceContextPropagator::new()
            .inject_context(&span.context(), &mut HeaderInjector(self.headers_mut()));

        self
    }
}
