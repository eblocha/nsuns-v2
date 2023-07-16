use axum::{
    routing::{delete, get},
    Router,
};

use crate::db::Pool;

use super::handler::{create_program, delete_program, update_program, user_programs};

pub fn router() -> Router<Pool> {
    Router::new()
        .route(
            "/",
            get(user_programs).post(create_program).put(update_program),
        )
        .route("/:id", delete(delete_program))
}
