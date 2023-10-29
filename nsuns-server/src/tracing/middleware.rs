use axum::{
    middleware::{from_fn, Next},
    response::IntoResponse,
    Router,
};
use tower_http::trace::TraceLayer;

use super::span::{OpenTelemetryRequestSpan, UpdateSpanOnResponse, WithSpan};

async fn propagate_otel_context<B>(req: http::Request<B>, next: Next<B>) -> impl IntoResponse {
    next.run(req).await.with_current_span()
}

pub trait WithTracing {
    fn with_tracing(self) -> Self;
}

impl<S> WithTracing for Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    fn with_tracing(self) -> Self {
        self.route_layer(from_fn(propagate_otel_context)).layer(
            TraceLayer::new_for_http()
                .make_span_with(OpenTelemetryRequestSpan)
                .on_response(UpdateSpanOnResponse),
        )
    }
}
