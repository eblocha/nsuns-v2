use anyhow::Result;
use axum::Router;
use tower_http::{
    cors::CorsLayer,
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
};
use tracing::Level;

use crate::{
    db::Pool, program::router::programs_router, settings::Settings, user::router::users_router,
};

pub fn router(pool: Pool, _settings: &Settings) -> Result<Router> {
    let app = Router::new()
        .nest("/api/programs", programs_router())
        .nest("/api/users", users_router())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
        .with_state(pool)
        .layer(CorsLayer::permissive());

    Ok(app)
}
