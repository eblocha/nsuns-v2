use axum::{routing::get, Router};

use crate::db::Pool;

use super::handler::{
    create_program, delete_program, profile_programs, program_summary, update_program,
};

pub fn router() -> Router<Pool> {
    Router::new()
        .route(
            "/",
            get(profile_programs)
                .post(create_program)
                .put(update_program),
        )
        .route("/:id", get(program_summary).delete(delete_program))
}
