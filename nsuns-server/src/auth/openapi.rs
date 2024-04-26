use const_format::formatcp;
use utoipa::openapi::{
    path::{OperationBuilder, PathItemBuilder},
    ComponentsBuilder, PathItem, PathItemType, PathsBuilder, ResponseBuilder,
};

use crate::{
    openapi::{extensions::no_content, Customizer},
    router::AUTH_PATH,
};

pub struct AuthModule;

pub const COOKIE_AUTH: &str = "cookie_auth";

const TAG: &str = "Auth";

impl Customizer<ComponentsBuilder> for AuthModule {
    fn customize(builder: ComponentsBuilder) -> ComponentsBuilder {
        builder
    }
}

fn create_auth_path() -> PathItem {
    let post_op = OperationBuilder::new()
        .response(no_content(), ResponseBuilder::new().build())
        .tag(TAG)
        .build();

    PathItemBuilder::new()
        .operation(PathItemType::Post, post_op)
        .build()
}

impl Customizer<PathsBuilder> for AuthModule {
    fn customize(builder: PathsBuilder) -> PathsBuilder {
        builder
            .path(formatcp!("{AUTH_PATH}/login"), create_auth_path())
            .path(formatcp!("{AUTH_PATH}/anonymous"), create_auth_path())
            .path(formatcp!("{AUTH_PATH}/logout"), create_auth_path())
    }
}
