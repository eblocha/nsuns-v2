use std::{
    fmt,
    time::{Duration, Instant},
};

use axum::{
    extract::Request, middleware::{from_fn, Next}, response::IntoResponse, Router
};
use http::Method;
use tower_http::{trace::TraceLayer, LatencyUnit};
use tracing_core::Level;

use super::span::{get_trace_id, OpenTelemetryRequestSpan, UpdateSpanOnResponse};

struct Latency {
    unit: LatencyUnit,
    duration: Duration,
}

impl Latency {
    fn new_dynamic_unit(duration: Duration) -> Self {
        let unit = if duration.as_secs() > 0 || duration.subsec_nanos() > 1_000_000 {
            LatencyUnit::Millis
        } else {
            LatencyUnit::Micros
        };

        Self { unit, duration }
    }
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

struct RequestMetadata {
    start: Instant,
    method: Method,
    path: String,
    query: Option<String>,
}

fn collect(req: &Request) -> RequestMetadata {
    RequestMetadata {
        start: Instant::now(),
        method: req.method().clone(),
        path: req.uri().path().to_owned(),
        query: req.uri().query().map(ToOwned::to_owned),
    }
}

pub async fn trace(request: Request, next: Next) -> impl IntoResponse {
    let meta = if tracing::enabled!(Level::INFO) {
        Some(collect(&request))
    } else {
        None
    };

    let response = next.run(request).await;

    if let Some(meta) = meta {
        let trace_id = get_trace_id(&tracing::Span::current());

        let latency = Latency::new_dynamic_unit(meta.start.elapsed());

        tracing::info!(
            %latency,
            status = response.status().as_u16(),
            method = %meta.method,
            path = meta.path,
            query = meta.query,
            trace_id,
            "finished processing request",
        );
    }
    response
}

pub trait WithTracing {
    #[must_use]
    fn with_tracing(self) -> Self;
}

impl<S> WithTracing for Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    fn with_tracing(self) -> Self {
        self.layer(from_fn(trace)).layer(
            TraceLayer::new_for_http()
                .make_span_with(OpenTelemetryRequestSpan)
                .on_response(UpdateSpanOnResponse)
                .on_failure(()),
        )
    }
}
