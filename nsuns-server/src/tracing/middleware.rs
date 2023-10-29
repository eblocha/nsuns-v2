use axum::{
    http::Request,
    middleware::{from_fn, Next},
    response::IntoResponse,
    Router,
};
use opentelemetry_api::propagation::TextMapPropagator;
use opentelemetry_http::HeaderInjector;
use opentelemetry_sdk::propagation::TraceContextPropagator;
use tower_http::trace::{DefaultOnResponse, TraceLayer};
use tracing::{Level, Span};
use tracing_opentelemetry::OpenTelemetrySpanExt;

use super::span::{DynamicLatencyUnitOnResponse, OpenTelemetryRequestSpan};

async fn propagate_otel_context<B>(req: Request<B>, next: Next<B>) -> impl IntoResponse {
    let mut response = next.run(req).await;

    TraceContextPropagator::new().inject_context(
        &Span::current().context(),
        &mut HeaderInjector(response.headers_mut()),
    );

    response
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
                .on_response(DynamicLatencyUnitOnResponse(
                    DefaultOnResponse::new().level(Level::INFO),
                )),
        )
    }
}
