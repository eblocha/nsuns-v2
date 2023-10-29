use tower_http::{
    trace::{DefaultOnResponse, MakeSpan, OnResponse},
    LatencyUnit,
};
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct RequestSpan;

impl<B> MakeSpan<B> for RequestSpan {
    fn make_span(&mut self, request: &hyper::Request<B>) -> tracing::Span {
        tracing::info_span!(
            "request",
            method = %request.method(),
            uri = %request.uri(),
            request_id = %Uuid::new_v4(),
            version = ?request.version(),
        )
    }
}

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
