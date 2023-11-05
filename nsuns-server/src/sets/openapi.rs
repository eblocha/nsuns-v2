use const_format::concatcp;
use utoipa::{
    openapi::{
        path::{OperationBuilder, PathItemBuilder},
        request_body::RequestBodyBuilder,
        response::Response,
        ComponentsBuilder, PathItemType, PathsBuilder, ResponseBuilder,
    },
    ToSchema,
};

use crate::{
    openapi::{
        extensions::{created, id_path_param, no_content, ok, JsonContent},
        Customizer,
    },
    router::SETS_PATH,
};

use super::model::{CreateSet, Set, UpdateSet};

pub struct SetsModule;

fn set_response() -> Response {
    ResponseBuilder::new().json_content(Set::schema().1).build()
}

const TAG: &str = "Sets";

impl Customizer<ComponentsBuilder> for SetsModule {
    fn customize(builder: ComponentsBuilder) -> ComponentsBuilder {
        builder
            .schema_from::<Set>()
            .schema_from::<CreateSet>()
            .schema_from::<UpdateSet>()
    }
}

impl Customizer<PathsBuilder> for SetsModule {
    fn customize(builder: PathsBuilder) -> PathsBuilder {
        let post_op = OperationBuilder::new()
            .request_body(Some(
                RequestBodyBuilder::new()
                    .json_content(CreateSet::schema().1)
                    .build(),
            ))
            .response(created(), set_response())
            .tag(TAG)
            .build();

        let put_op = OperationBuilder::new()
            .request_body(Some(
                RequestBodyBuilder::new()
                    .json_content(UpdateSet::schema().1)
                    .build(),
            ))
            .response(ok(), set_response())
            .tag(TAG)
            .build();

        let delete_op = OperationBuilder::new()
            .parameters(id_path_param(Some("The set to delete")))
            .response(no_content(), Response::new("no content"))
            .tag(TAG)
            .build();

        builder
            .path(
                SETS_PATH,
                PathItemBuilder::new()
                    .operation(PathItemType::Post, post_op)
                    .operation(PathItemType::Put, put_op)
                    .build(),
            )
            .path(
                concatcp!(SETS_PATH, "/{id}"),
                PathItemBuilder::new()
                    .operation(PathItemType::Delete, delete_op)
                    .build(),
            )
    }
}
