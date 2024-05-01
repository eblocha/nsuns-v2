use axum::Router;
use utoipa::{
    openapi::{ComponentsBuilder, Info, OpenApiBuilder, PathsBuilder},
    OpenApi,
};

use crate::{
    auth::openapi::AuthModule, maxes::openapi::MaxesModule, movements::openapi::MovementsModule,
    profiles::openapi::ProfilesModule, program::openapi::ProgramModule, reps::openapi::RepsModule,
    sets::openapi::SetsModule, updates::openapi::UpdatesModule,
};

use self::settings::OpenApiFeature;

pub mod extensions;
pub mod settings;

pub struct ApiDoc;

trait WithModule<B> {
    fn with_module<T: Customizer<B>>(self) -> Self;
}

pub trait Customizer<T> {
    fn customize(builder: T) -> T;
}

impl<B> WithModule<B> for B {
    fn with_module<T: Customizer<B>>(self) -> Self {
        T::customize(self)
    }
}

impl OpenApi for ApiDoc {
    fn openapi() -> utoipa::openapi::OpenApi {
        let paths = PathsBuilder::new()
            .with_module::<AuthModule>()
            .with_module::<MaxesModule>()
            .with_module::<MovementsModule>()
            .with_module::<ProfilesModule>()
            .with_module::<ProgramModule>()
            .with_module::<RepsModule>()
            .with_module::<SetsModule>()
            .with_module::<UpdatesModule>()
            .build();

        let components = ComponentsBuilder::new()
            .with_module::<AuthModule>()
            .with_module::<MaxesModule>()
            .with_module::<MovementsModule>()
            .with_module::<ProfilesModule>()
            .with_module::<ProgramModule>()
            .with_module::<RepsModule>()
            .with_module::<SetsModule>()
            .with_module::<UpdatesModule>()
            .build();

        OpenApiBuilder::new()
            .info(Info::new("NSuns Server", "1"))
            .paths(paths)
            .components(Some(components))
            .build()
    }
}

pub trait WithOpenApi {
    #[must_use]
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
