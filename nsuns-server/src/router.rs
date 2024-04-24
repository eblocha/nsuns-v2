use std::path::Path;

use axum::{middleware::from_fn_with_state, routing::get, Router};
use axum_macros::FromRef;
use tower_cookies::CookieManagerLayer;
use tower_http::{
    catch_panic::CatchPanicLayer,
    services::{ServeDir, ServeFile},
};

use crate::{
    auth::{self, middleware::manage_tokens, token::JwtKeys},
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
pub const AUTH_PATH: &str = "/api/auth";

trait StaticFiles<P> {
    fn static_files(self, static_dir: Option<P>) -> Self;
}

impl<S, P> StaticFiles<P> for Router<S>
where
    S: Clone + Send + Sync + 'static,
    P: AsRef<Path>,
{
    fn static_files(self, static_dir: Option<P>) -> Self {
        if let Some(ref static_dir) = static_dir {
            let mut path_buf = static_dir.as_ref().to_path_buf();
            path_buf.push("index.html");

            let serve_dir = ServeDir::new(static_dir)
                .precompressed_gzip()
                .precompressed_br()
                .precompressed_deflate()
                .not_found_service(ServeFile::new(path_buf));

            self.fallback_service(serve_dir)
        } else {
            self
        }
    }
}

#[derive(Clone, FromRef)]
pub struct AppState {
    pub pool: Pool,
    pub keys: JwtKeys,
}

pub trait State: Clone + Send + Sync + 'static {}

impl<S> State for S where S: Clone + Send + Sync + 'static {}

pub fn router(state: AppState, settings: &Settings) -> anyhow::Result<Router> {
    Ok(Router::new()
        .nest(PROFILES_PATH, profiles::router())
        .nest(PROGRAMS_PATH, program::router())
        .nest(SETS_PATH, sets::router())
        .nest(MOVEMENTS_PATH, movements::router())
        .nest(MAXES_PATH, maxes::router())
        .nest(REPS_PATH, reps::router())
        .nest(UPDATES_PATH, updates::router())
        .nest(AUTH_PATH, auth::router())
        .with_state(state.clone())
        .route_layer(from_fn_with_state(state.clone(), manage_tokens))
        .layer(CookieManagerLayer::new())
        .with_openapi(&settings.openapi)
        .layer(CatchPanicLayer::new())
        .static_files(settings.server.static_dir.as_ref())
        .route(HEALTH_PATH, get(health_check))
        .with_metrics(&settings.metrics)
        .with_tracing())
}
