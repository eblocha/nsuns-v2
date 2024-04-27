use const_format::formatcp;
use utoipa::{
    openapi::{
        path::{OperationBuilder, PathItemBuilder},
        security::{Http, HttpAuthScheme, SecurityScheme},
        ComponentsBuilder, PathItem, PathItemType, PathsBuilder, ResponseBuilder,
        SecurityRequirement,
    },
    ToSchema,
};

use crate::{
    openapi::{
        extensions::{no_content, ok, JsonContent},
        Customizer,
    },
    router::AUTH_PATH,
};

use super::user::UserInfo;

pub struct AuthModule;

pub const BASIC_AUTH: &str = "basic_auth";

const TAG: &str = "Auth";

impl Customizer<ComponentsBuilder> for AuthModule {
    fn customize(builder: ComponentsBuilder) -> ComponentsBuilder {
        builder.security_scheme(
            BASIC_AUTH,
            SecurityScheme::Http(Http::new(HttpAuthScheme::Basic)),
        )
    }
}

fn create_auth_path(description: &str) -> PathItem {
    let post_op = OperationBuilder::new()
        .description(Some(description))
        .response(no_content(), ResponseBuilder::new().build())
        .tag(TAG)
        .build();

    PathItemBuilder::new()
        .operation(PathItemType::Post, post_op)
        .build()
}

impl Customizer<PathsBuilder> for AuthModule {
    fn customize(builder: PathsBuilder) -> PathsBuilder {
        let login_op = OperationBuilder::new()
            .description(Some("Log in as a persistent user"))
            .response(no_content(), ResponseBuilder::new().build())
            .security(SecurityRequirement::new::<_, _, &str>(BASIC_AUTH, []))
            .tag(TAG)
            .build();

        builder
            .path(
                formatcp!("{AUTH_PATH}/login"),
                PathItemBuilder::new()
                    .operation(PathItemType::Post, login_op)
                    .build(),
            )
            .path(
                formatcp!("{AUTH_PATH}/anonymous"),
                create_auth_path("Log in as a temporary, anonymous user"),
            )
            .path(
                formatcp!("{AUTH_PATH}/logout"),
                create_auth_path("Log out. This will delete data if the login is anonymous"),
            )
            .path(
                formatcp!("{AUTH_PATH}/user-info"),
                PathItemBuilder::new()
                    .operation(
                        PathItemType::Get,
                        OperationBuilder::new()
                            .description(Some(
                                "Get information about the user you are logged-in as",
                            ))
                            .response(
                                ok(),
                                ResponseBuilder::new()
                                    .description(
                                        "Information about your user, or null of anonymous",
                                    )
                                    .json_content(UserInfo::schema().1),
                            )
                            .tag(TAG)
                            .build(),
                    )
                    .build(),
            )
    }
}
