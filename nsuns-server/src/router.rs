use std::{fmt::Display, path::Path};

use axum::{middleware, Router};
use serde::Deserialize;
use tower_http::{
    catch_panic::CatchPanicLayer,
    services::{ServeDir, ServeFile},
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
};
use tracing::Level;

use crate::{
    db::Pool,
    maxes::router::maxes_router,
    metrics::middleware::track_metrics,
    movements::router::movements_router,
    profiles::router::profiles_router,
    program::router::programs_router,
    reps::router::reps_router,
    sets::router::sets_router,
    settings::{OpenApiFeature, Settings},
    updates::router::updates_router,
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

fn default_swagger_path() -> String {
    "/swagger-ui".to_string()
}

fn default_openapi_path() -> String {
    "/api-docs/openapi.json".to_string()
}

#[derive(Debug, Deserialize)]
pub struct OpenApiSettings {
    #[serde(default = "default_swagger_path")]
    pub swagger_path: String,
    #[serde(default = "default_openapi_path")]
    pub openapi_path: String,
}

impl Default for OpenApiSettings {
    fn default() -> Self {
        Self {
            swagger_path: default_swagger_path(),
            openapi_path: default_openapi_path(),
        }
    }
}

trait WithOpenApi {
    fn with_openapi(self, settings: &OpenApiFeature) -> Self;
}

impl<S> WithOpenApi for Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    #[cfg(feature = "openapi")]
    fn with_openapi(self, settings: &OpenApiFeature) -> Self {
        use crate::{feature::Feature, openapi::ApiDoc};
        use utoipa::OpenApi;
        use utoipa_swagger_ui::SwaggerUi;

        if let Feature::Enabled(config) = settings {
            self.merge(
                SwaggerUi::new(config.swagger_path.clone())
                    .url(config.openapi_path.clone(), ApiDoc::openapi()),
            )
        } else {
            self
        }
    }

    #[cfg(not(feature = "openapi"))]
    fn with_openapi(self, _settings: &OpenApiFeature) -> Self {
        self
    }
}

pub fn router(pool: Pool, settings: &Settings) -> Router {
    let app = Router::new()
        .nest(PROFILES_PATH, profiles_router())
        .nest(PROGRAMS_PATH, programs_router())
        .nest(SETS_PATH, sets_router())
        .nest(MOVEMENTS_PATH, movements_router())
        .nest(MAXES_PATH, maxes_router())
        .nest(REPS_PATH, reps_router())
        .nest(UPDATES_PATH, updates_router())
        .with_openapi(&settings.openapi)
        .layer(CatchPanicLayer::new())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
        .with_state(pool)
        .static_files(settings.server.static_dir.as_ref());

    if settings.metrics.is_enabled() {
        app.route_layer(middleware::from_fn(track_metrics))
    } else {
        app
    }
}
