use axum::Router;
use utoipa::{
    openapi::{ComponentsBuilder, Info, OpenApiBuilder, PathsBuilder},
    OpenApi,
};

use crate::{
    maxes::openapi::WithMaxesDefinition, movements::openapi::WithMovementsDefinition,
    profiles::openapi::WithProfilesDefinition, program::openapi::WithProgramsDefintions,
    reps::openapi::WithRepsDefinition, sets::openapi::WithSetsDefinition,
    updates::openapi::WithUpdatesDefinition,
};

use self::settings::OpenApiFeature;

pub mod extensions;
pub mod settings;

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

pub trait WithOpenApi {
    fn with_openapi(self, settings: &OpenApiFeature) -> Self;
}

impl<S> WithOpenApi for Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    #[cfg(feature = "openapi")]
    fn with_openapi(self, settings: &OpenApiFeature) -> Self {
        use utoipa_swagger_ui::SwaggerUi;

        if let OpenApiFeature::Enabled(config) = settings {
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
