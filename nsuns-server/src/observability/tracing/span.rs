use axum::extract::MatchedPath;
use opentelemetry::{propagation::TextMapPropagator, trace::TraceContextExt};
use opentelemetry_api::trace::SpanKind;
use opentelemetry_http::HeaderExtractor;
use opentelemetry_sdk::propagation::TraceContextPropagator;
use opentelemetry_semantic_conventions as semcov;
use tower_http::trace::{MakeSpan, OnResponse};
use tracing::field::Empty;
use tracing_opentelemetry::OpenTelemetrySpanExt;

use crate::observability::attributes::HttpRequestAttributes;

/// Set the request span's parent as the incoming otel span, if present.
#[derive(Debug, Clone)]
pub struct OpenTelemetryRequestSpan;

impl<B> MakeSpan<B> for OpenTelemetryRequestSpan {
    fn make_span(&mut self, request: &http::Request<B>) -> tracing::Span {
        let extractor = HeaderExtractor(request.headers());

        let parent = TraceContextPropagator::new().extract(&extractor);

        let attrs: HttpRequestAttributes = request.into();

        let matched_path = request
            .extensions()
            .get::<MatchedPath>()
            .map(axum::extract::MatchedPath::as_str);

        let method_verb = attrs.http_request_method.as_str();

        let span_name =
            matched_path.map_or_else(|| method_verb.to_owned(), |m| format!("{method_verb} {m}"));

        let user_agent_original = attrs.user_agent_original;
        let client_address = attrs.client_address;
        let network_local_addr = attrs.network_local_address.map(|ip| ip.to_string());
        let network_local_port = attrs.network_local_port;
        let network_peer_addr = attrs.network_peer_address.map(|ip| ip.to_string());
        let network_peer_port = attrs.network_peer_port;
        let network_protocol_name = attrs.network_protocol_name;
        let network_protocol_version = attrs.network_protocol_version;
        let network_type = attrs.network_type;
        let server_address = attrs.server_address;
        let url_scheme = attrs.url_scheme;

        let tracing_span = tracing::info_span!(
            "HTTP request",
            otel.kind = ?SpanKind::Server,
            otel.name = span_name,
            http.request.method = method_verb,
            http.route = matched_path,
            user_agent.original = user_agent_original,
            client.address = client_address,
            network.local.address = network_local_addr,
            network.local.port = network_local_port,
            network.peer.address = network_peer_addr,
            network.peer.port = network_peer_port,
            network.protocol.name = network_protocol_name,
            network.protocol.version = network_protocol_version,
            "network.type" = network_type,
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
            semcov::trace::HTTP_RESPONSE_STATUS_CODE,
            response.status().as_u16(),
        );

        if response.status().is_server_error() {
            span.record(semcov::trace::OTEL_STATUS_CODE, "ERROR");
        }

        span.record(
            semcov::attribute::HTTP_RESPONSE_BODY_SIZE,
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
