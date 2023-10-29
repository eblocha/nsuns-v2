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
            http.response.body.size = Empty
        );

        tracing_span.set_parent(parent);

        tracing_span
    }
}

/// Updates the request span with response information and logs a request event.
#[derive(Debug, Clone)]
pub struct UpdateSpanOnResponse(pub DefaultOnResponse);

impl<B> OnResponse<B> for UpdateSpanOnResponse
{
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

        self.0
            .latency_unit(unit)
            .on_response(response, latency, span)
    }
}

/// Update the given span to record fields from the http response
pub fn update_span_from_response<B>(span: &tracing::Span, response: &http::Response<B>)
{
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

    span.record(
        semcov::trace::HTTP_RESPONSE_BODY_SIZE.as_str(),
        response_body_size(response),
    );
}

#[inline]
fn user_agent<B>(req: &http::Request<B>) -> &str {
    req.headers()
        .get(http::header::USER_AGENT)
        .map_or("", |h| h.to_str().unwrap_or_default())
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
