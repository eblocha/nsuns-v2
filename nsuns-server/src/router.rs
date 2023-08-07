use std::{fmt::Display, path::Path};

use axum::{middleware, Router};
use tower_http::{
    catch_panic::CatchPanicLayer,
    services::{ServeDir, ServeFile},
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
};
use tracing::Level;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    db::Pool, maxes::router::maxes_router, metrics::middleware::track_metrics,
    movements::router::movements_router, openapi::ApiDoc, profiles::router::profiles_router,
    program::router::programs_router, reps::router::reps_router, sets::router::sets_router,
    settings::Settings, updates::router::updates_router,
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
                .not_found_service(ServeFile::new(format!("{static_dir}/index.html")));

            self.fallback_service(serve_dir)
        } else {
            self
        }
    }
}

pub fn router(pool: Pool, settings: &Settings) -> Router {
    Router::new()
        .nest(PROFILES_PATH, profiles_router())
        .nest(PROGRAMS_PATH, programs_router())
        .nest(SETS_PATH, sets_router())
        .nest(MOVEMENTS_PATH, movements_router())
        .nest(MAXES_PATH, maxes_router())
        .nest(REPS_PATH, reps_router())
        .nest(UPDATES_PATH, updates_router())
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(CatchPanicLayer::new())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
        .with_state(pool)
        .static_files(settings.server.static_dir.as_ref())
        .route_layer(middleware::from_fn(track_metrics))
}
