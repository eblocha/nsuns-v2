use std::{
    fmt,
    time::{Duration, Instant},
};

use axum::{
    middleware::{from_fn, Next},
    response::IntoResponse,
    Router,
};
use tower_http::{trace::TraceLayer, LatencyUnit};

use crate::tracing::span::{get_trace_id, WithSpan};

use super::span::{OpenTelemetryRequestSpan, UpdateSpanOnResponse};

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

async fn trace<B>(req: http::Request<B>, next: Next<B>) -> impl IntoResponse {
    let start = Instant::now();

    // info to be logged
    let method = req.method().to_string();
    let path = req.uri().path().to_string();
    let query = req.uri().query().map(ToString::to_string);

    let res = next.run(req).await.with_current_span();

    let trace_id = get_trace_id(&tracing::Span::current());

    let latency = Latency::new_dynamic_unit(start.elapsed());

    tracing::info!(
        %latency,
        status = res.status().as_u16(),
        method,
        path,
        query,
        trace_id,
        "finished processing request",
    );

    res
}

pub trait WithTracing {
    fn with_tracing(self) -> Self;
}

impl<S> WithTracing for Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    fn with_tracing(self) -> Self {
        self.route_layer(from_fn(trace)).layer(
            TraceLayer::new_for_http()
                .make_span_with(OpenTelemetryRequestSpan)
                .on_response(UpdateSpanOnResponse),
        )
    }
}
