use anyhow::Result;
use axum::Router;
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
};
use tracing::Level;

use crate::{
    db::Pool, maxes::router::maxes_router, movements::router::movements_router,
    profiles::router::profiles_router, program::router::programs_router, reps::router::reps_router,
    sets::router::sets_router, settings::Settings, updates::router::updates_router,
};

pub fn router(pool: Pool, settings: &Settings) -> Result<Router> {
    let app = Router::new()
        .nest("/api/profiles", profiles_router())
        .nest("/api/programs", programs_router())
        .nest("/api/sets", sets_router())
        .nest("/api/movements", movements_router())
        .nest("/api/maxes", maxes_router())
        .nest("/api/reps", reps_router())
        .nest("/api/updates", updates_router())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
        .with_state(pool);

    if let Some(ref static_dir) = settings.server.static_dir {
        let serve_dir = ServeDir::new(static_dir).not_found_service(ServeFile::new(
            format!("{}/index.html", static_dir),
        ));

        Ok(app.fallback_service(serve_dir))
    } else {
        Ok(app)
    }

}
