use axum::Router;
use tower_http::trace::{DefaultOnResponse, TraceLayer};
use tracing::Level;

use super::request_span::{DynamicLatencyUnitOnResponse, RequestSpan};

pub trait WithTracing {
    fn with_tracing(self) -> Self;
}

impl<S> WithTracing for Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    fn with_tracing(self) -> Self {
        self.layer(
            TraceLayer::new_for_http()
                .make_span_with(RequestSpan)
                .on_response(DynamicLatencyUnitOnResponse(
                    DefaultOnResponse::new().level(Level::INFO),
                )),
        )
    }
}
