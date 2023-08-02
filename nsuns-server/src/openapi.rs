use axum::http::StatusCode;
use utoipa::{
    openapi::{
        path::{Parameter, ParameterBuilder, ParameterIn},
        ComponentsBuilder, Info, OpenApiBuilder, PathsBuilder, Required,
    },
    OpenApi,
};

use crate::{
    maxes::openapi::WithMaxesDefinition, movements::openapi::WithMovementsDefinition,
    profiles::openapi::WithProfilesDefinition, program::openapi::WithProgramsDefintions,
    reps::openapi::WithRepsDefinition, sets::openapi::WithSetsDefinition,
    updates::openapi::WithUpdatesDefinition,
};

pub const APPLICATION_JSON: &str = "application/json";

pub fn ok() -> &'static str {
    StatusCode::OK.as_str()
}

pub fn created() -> &'static str {
    StatusCode::CREATED.as_str()
}

pub fn no_content() -> &'static str {
    StatusCode::NO_CONTENT.as_str()
}

pub fn param_in_default() -> Option<ParameterIn> {
    None
}

pub fn id_path_param(description: Option<&str>) -> Option<Vec<Parameter>> {
    Some(vec![ParameterBuilder::new()
        .name("id")
        .description(description)
        .required(Required::True)
        .parameter_in(ParameterIn::Path)
        .build()])
}

pub struct ApiDoc;

impl OpenApi for ApiDoc {
    fn openapi() -> utoipa::openapi::OpenApi {
        let paths = PathsBuilder::new()
            .with_maxes()
            .with_movements()
            .with_profiles()
            .with_programs()
            .with_reps()
            .with_sets()
            .with_updates()
            .build();

        let components = ComponentsBuilder::new()
            .with_maxes()
            .with_movements()
            .with_profiles()
            .with_programs()
            .with_reps()
            .with_sets()
            .with_updates()
            .build();

        OpenApiBuilder::new()
            .info(Info::new("NSuns Server", "1"))
            .paths(paths)
            .components(Some(components))
            .build()
    }
}
