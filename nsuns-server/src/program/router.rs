use axum::{routing::get, Router};

use crate::db::Pool;

use super::handler::{
    create_program, delete_program, program_summary, update_program, user_programs,
};

pub fn programs_router() -> Router<Pool> {
    Router::new()
        .route(
            "/",
            get(user_programs).post(create_program).put(update_program),
        )
        .route("/:id", get(program_summary).delete(delete_program))
}
