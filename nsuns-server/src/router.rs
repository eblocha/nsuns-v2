use std::{fmt::Display, path::Path};

use axum::Router;
use tower_http::{
    catch_panic::CatchPanicLayer,
    services::{ServeDir, ServeFile},
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer}, compression::{CompressionLayer, predicate::SizeAbove},
};
use tracing::Level;

use crate::{
    db::Pool, maxes, metrics::middleware::WithMetrics, movements, openapi::WithOpenApi, profiles,
    program, reps, sets, settings::Settings, updates,
};

pub const PROFILES_PATH: &str = "/api/profiles";
pub const PROGRAMS_PATH: &str = "/api/programs";
pub const SETS_PATH: &str = "/api/sets";
pub const MOVEMENTS_PATH: &str = "/api/movements";
pub const MAXES_PATH: &str = "/api/maxes";
pub const REPS_PATH: &str = "/api/reps";
pub const UPDATES_PATH: &str = "/api/updates";

trait StaticFiles<P> {
    fn static_files(self, static_dir: Option<P>) -> Self;
}

impl<S, P> StaticFiles<P> for Router<S>
where
    S: Clone + Send + Sync + 'static,
    P: AsRef<Path> + Display,
{
    fn static_files(self, static_dir: Option<P>) -> Self {
        if let Some(ref static_dir) = static_dir {
            let serve_dir = ServeDir::new(static_dir)
                .precompressed_gzip()
                .precompressed_br()
                .precompressed_deflate()
                .not_found_service(ServeFile::new(format!("{static_dir}/index.html")));

            self.fallback_service(serve_dir)
        } else {
            self
        }
    }
}

pub fn router(pool: Pool, settings: &Settings) -> Router {
    Router::new()
        .nest(PROFILES_PATH, profiles::router())
        .nest(PROGRAMS_PATH, program::router())
        .nest(SETS_PATH, sets::router())
        .nest(MOVEMENTS_PATH, movements::router())
        .nest(MAXES_PATH, maxes::router())
        .nest(REPS_PATH, reps::router())
        .nest(UPDATES_PATH, updates::router())
        .layer(CompressionLayer::new().compress_when(SizeAbove::new(1024)))
        .with_openapi(&settings.openapi)
        .layer(CatchPanicLayer::new())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
        .with_state(pool)
        .static_files(settings.server.static_dir.as_ref())
        .with_metrics(&settings.metrics)
}
