use axum::{
    routing::{get, post},
    Router,
};

use crate::db::Pool;

use super::handler::{
    create_program, delete_program, profile_programs, program_summary, reorder_sets, update_program,
};

pub const REORDER_SETS_PATH: &str = "/reorder-sets";

pub fn router() -> Router<Pool> {
    Router::new()
        .route(
            "/",
            get(profile_programs)
                .post(create_program)
                .put(update_program),
        )
        .route(REORDER_SETS_PATH, post(reorder_sets))
        .route("/:id", get(program_summary).delete(delete_program))
}
