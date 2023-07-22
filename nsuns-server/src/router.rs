use anyhow::Result;
use axum::Router;
use tower_http::{
    cors::CorsLayer,
    services::{ServeDir, ServeFile},
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
};
use tracing::Level;

use crate::{
    db::Pool, maxes::router::maxes_router, movements::router::movements_router,
    profiles::router::profiles_router, program::router::programs_router, sets::router::sets_router,
    settings::Settings,
};

pub fn router(pool: Pool, _settings: &Settings) -> Result<Router> {
    let serve_dir = ServeDir::new("static").not_found_service(ServeFile::new("static/assets/index.html"));

    let app = Router::new()
        .nest("/api/profiles", profiles_router())
        .nest("/api/programs", programs_router())
        .nest("/api/sets", sets_router())
        .nest("/api/movements", movements_router())
        .nest("/api/maxes", maxes_router())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
        .with_state(pool)
        .layer(CorsLayer::permissive())
        .fallback_service(serve_dir);

    Ok(app)
}
