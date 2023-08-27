use const_format::concatcp;
use utoipa::{
    openapi::{
        path::{OperationBuilder, PathItemBuilder},
        request_body::RequestBodyBuilder,
        response::Response,
        ComponentsBuilder, PathItemType, PathsBuilder, ResponseBuilder,
    },
    PartialSchema, ToSchema,
};

use crate::{
    openapi::extensions::{created, id_path_param, ok, JsonContent},
    router::PROFILES_PATH,
};

use super::model::{CreateProfile, Profile};

pub trait WithProfilesDefinition {
    fn with_profiles(self) -> Self;
}

impl WithProfilesDefinition for ComponentsBuilder {
    fn with_profiles(self) -> Self {
        self.schema_from::<Profile>().schema_from::<CreateProfile>()
    }
}

fn profile_response() -> Response {
    ResponseBuilder::new()
        .json_content(Profile::schema().1)
        .build()
}

const TAG: &str = "Profiles";

impl WithProfilesDefinition for PathsBuilder {
    fn with_profiles(self) -> Self {
        let get_index_op = OperationBuilder::new()
            .response(
                ok(),
                ResponseBuilder::new()
                    .json_content(Vec::<Profile>::schema())
                    .build(),
            )
            .tag(TAG)
            .build();

        let get_one_op = OperationBuilder::new()
            .parameters(id_path_param(Some("The id of the profile to fetch")))
            .response(ok(), profile_response())
            .tag(TAG)
            .build();

        let post_op = OperationBuilder::new()
            .request_body(Some(
                RequestBodyBuilder::new()
                    .json_content(CreateProfile::schema().1)
                    .build(),
            ))
            .response(created(), profile_response())
            .tag(TAG)
            .build();

        let put_op = OperationBuilder::new()
            .request_body(Some(
                RequestBodyBuilder::new()
                    .json_content(Profile::schema().1)
                    .build(),
            ))
            .response(ok(), profile_response())
            .tag(TAG)
            .build();

        let delete_op = OperationBuilder::new()
            .parameters(id_path_param(Some("The id of the profile to delete")))
            .response(ok(), profile_response())
            .tag(TAG)
            .build();

        self.path(
            PROFILES_PATH,
            PathItemBuilder::new()
                .operation(PathItemType::Get, get_index_op)
                .operation(PathItemType::Post, post_op)
                .operation(PathItemType::Put, put_op)
                .build(),
        )
        .path(
            concatcp!(PROFILES_PATH, "/{id}"),
            PathItemBuilder::new()
                .operation(PathItemType::Get, get_one_op)
                .operation(PathItemType::Delete, delete_op)
                .build(),
        )
    }
}
