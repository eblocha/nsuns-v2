use std::{fmt::Display, path::Path};

use axum::{routing::get, Router};
use tower_http::{
    catch_panic::CatchPanicLayer,
    services::{ServeDir, ServeFile},
};

use crate::{
    db::Pool,
    health::health_check,
    maxes, movements,
    observability::{metrics::middleware::WithMetrics, tracing::middleware::WithTracing},
    openapi::WithOpenApi,
    profiles, program, reps, sets,
    settings::Settings,
    updates,
};

pub const PROFILES_PATH: &str = "/api/profiles";
pub const PROGRAMS_PATH: &str = "/api/programs";
pub const SETS_PATH: &str = "/api/sets";
pub const MOVEMENTS_PATH: &str = "/api/movements";
pub const MAXES_PATH: &str = "/api/maxes";
pub const REPS_PATH: &str = "/api/reps";
pub const UPDATES_PATH: &str = "/api/updates";
pub const HEALTH_PATH: &str = "/actuator/health";

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

pub fn router(pool: Pool, settings: &Settings) -> anyhow::Result<Router> {
    Ok(Router::new()
        .nest(PROFILES_PATH, profiles::router())
        .nest(PROGRAMS_PATH, program::router())
        .nest(SETS_PATH, sets::router())
        .nest(MOVEMENTS_PATH, movements::router())
        .nest(MAXES_PATH, maxes::router())
        .nest(REPS_PATH, reps::router())
        .nest(UPDATES_PATH, updates::router())
        .with_state(pool)
        .route(HEALTH_PATH, get(health_check))
        .with_openapi(&settings.openapi)
        .layer(CatchPanicLayer::new())
        .static_files(settings.server.static_dir.as_ref())
        .with_tracing()
        .with_metrics(&settings.metrics))
}
