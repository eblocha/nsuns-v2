use std::net::IpAddr;

use axum::extract::{ConnectInfo, MatchedPath};
use http::Version;
use opentelemetry_api::{
    propagation::TextMapPropagator,
    trace::{SpanKind, TraceContextExt},
};
use opentelemetry_http::{HeaderExtractor, HeaderInjector};
use opentelemetry_sdk::propagation::TraceContextPropagator;
use opentelemetry_semantic_conventions as semcov;
use tower_http::trace::{MakeSpan, OnResponse};
use tracing::{field::Empty, Span};
use tracing_opentelemetry::OpenTelemetrySpanExt;

use crate::server::ClientInfo;

/// Set the request span's parent as the incoming otel span, if present.
#[derive(Debug, Clone)]
pub struct OpenTelemetryRequestSpan;

impl<B> MakeSpan<B> for OpenTelemetryRequestSpan {
    fn make_span(&mut self, request: &http::Request<B>) -> tracing::Span {
        let extractor = HeaderExtractor(request.headers());

        let parent = TraceContextPropagator::new().extract(&extractor);

        let matched_path = request
            .extensions()
            .get::<MatchedPath>()
            .map(|path| path.as_str());

        let method_verb = request.method().as_str();

        let span_name = matched_path
            .map(|m| format!("{method_verb} {m}"))
            .unwrap_or_else(|| method_verb.to_owned());

        let client_info = request
            .extensions()
            .get::<ConnectInfo<ClientInfo>>()
            .map(|c| c.0.clone());

        let peer_address = client_info
            .as_ref()
            .map(network_peer_ip)
            .map(|ip| ip.to_string());

        let client_address = forwarded_for(request).or(peer_address.as_deref());

        let server_address = http_host(request).or_else(|| forwarded_host(request));

        let url_scheme = request
            .uri()
            .scheme_str()
            .or_else(|| forwarded_proto(request));

        let tracing_span = tracing::info_span!(
            "HTTP request",
            otel.kind = ?SpanKind::Server,
            otel.name = span_name,
            http.request.method = method_verb,
            http.route = matched_path,
            user_agent.original = user_agent(request),
            client.address = client_address,
            network.local.address = client_info.as_ref().map(network_local_ip).map(|ip| ip.to_string()),
            network.local.port = client_info.as_ref().map(network_local_port),
            network.peer.address = peer_address,
            network.peer.port = client_info.as_ref().map(network_peer_port),
            network.protocol.name = "http",
            network.protocol.version = protocol_version(request),
            "network.type" = client_info.as_ref().map(network_type),
            server.address = server_address,
            url.path = request.uri().path(),
            url.query = request.uri().query(),
            url.scheme = url_scheme,
            // set in response
            http.response.status_code = Empty,
            otel.status_code = Empty,
            otel.status_description = Empty,
            http.response.body.size = Empty,
            // auth
            enduser.id = Empty,
            enduser.role = Empty,
            enduser.scope = Empty,
            session.id = Empty,
        );

        tracing_span.set_parent(parent);

        tracing_span
    }
}

/// Updates the request span with response information and logs a request event.
#[derive(Debug, Clone)]
pub struct UpdateSpanOnResponse;

impl<B> OnResponse<B> for UpdateSpanOnResponse {
    fn on_response(
        self,
        response: &http::Response<B>,
        _latency: std::time::Duration,
        span: &tracing::Span,
    ) {
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
    }
}

pub fn get_trace_id(span: &tracing::Span) -> Option<String> {
    let cx = span.context();
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
        .and_then(|h| h.to_str().ok())
}

#[inline]
fn response_body_size<B>(res: &http::Response<B>) -> Option<u64> {
    res.headers()
        .get(http::header::CONTENT_LENGTH)
        .and_then(|h| {
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

// Client Info

#[inline]
fn network_peer_ip(client_info: &ClientInfo) -> IpAddr {
    client_info.remote_addr.ip()
}

#[inline]
fn network_peer_port(client_info: &ClientInfo) -> u16 {
    client_info.remote_addr.port()
}

#[inline]
fn network_local_ip(client_info: &ClientInfo) -> IpAddr {
    client_info.local_addr.ip()
}

#[inline]
fn network_local_port(client_info: &ClientInfo) -> u16 {
    client_info.local_addr.port()
}

#[inline]
fn network_type(client_info: &ClientInfo) -> &'static str {
    if network_peer_ip(client_info).is_ipv4() {
        "ipv4"
    } else {
        "ipv6"
    }
}

fn forwarded_for<B>(req: &http::Request<B>) -> Option<&str> {
    req.headers()
        .get("X-Forwarded-For")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.split(',').next())
}

fn forwarded_host<B>(req: &http::Request<B>) -> Option<&str> {
    req.headers()
        .get("X-Forwarded-Host")
        .and_then(|h| h.to_str().ok())
}

fn forwarded_proto<B>(req: &http::Request<B>) -> Option<&str> {
    req.headers()
        .get("X-Forwarded-Proto")
        .and_then(|h| h.to_str().ok())
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
