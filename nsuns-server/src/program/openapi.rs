use const_format::concatcp;
use utoipa::{
    openapi::{
        path::{OperationBuilder, PathItemBuilder},
        request_body::RequestBodyBuilder,
        response::Response,
        ComponentsBuilder, PathItemType, PathsBuilder, ResponseBuilder,
    },
    IntoParams, PartialSchema, ToSchema,
};

use crate::{
    openapi::{
        extensions::{created, id_path_param, ok, param_in_default, JsonContent},
        Customizer,
    },
    router::PROGRAMS_PATH,
};

use super::{
    handler::ProgramQuery,
    model::{CreateProgram, ProgramMeta, ProgramSummary, ReorderSets, SetId, UpdateProgram},
    router::REORDER_SETS_PATH,
};

pub struct ProgramModule;

fn program_response() -> Response {
    ResponseBuilder::new()
        .json_content(ProgramMeta::schema().1)
        .build()
}

const TAG: &str = "Programs";

impl Customizer<ComponentsBuilder> for ProgramModule {
    fn customize(builder: ComponentsBuilder) -> ComponentsBuilder {
        builder
            .schema_from::<ProgramMeta>()
            .schema_from::<CreateProgram>()
            .schema_from::<UpdateProgram>()
            .schema_from::<ProgramSummary>()
    }
}

impl Customizer<PathsBuilder> for ProgramModule {
    fn customize(builder: PathsBuilder) -> PathsBuilder {
        let get_op = OperationBuilder::new()
            .parameters(Some(ProgramQuery::into_params(param_in_default)))
            .response(
                ok(),
                ResponseBuilder::new()
                    .json_content(Vec::<ProgramMeta>::schema())
                    .build(),
            )
            .tag(TAG)
            .build();

        let post_op = OperationBuilder::new()
            .request_body(Some(
                RequestBodyBuilder::new()
                    .json_content(CreateProgram::schema().1)
                    .build(),
            ))
            .response(created(), program_response())
            .tag(TAG)
            .build();

        let put_op = OperationBuilder::new()
            .request_body(Some(
                RequestBodyBuilder::new()
                    .json_content(UpdateProgram::schema().1)
                    .build(),
            ))
            .response(ok(), program_response())
            .tag(TAG)
            .build();

        let delete_op = OperationBuilder::new()
            .parameters(id_path_param(Some("The id of the program to delete")))
            .response(ok(), program_response())
            .tag(TAG)
            .build();

        let summary_op = OperationBuilder::new()
            .parameters(id_path_param(Some(
                "The id of the program to fetch a summary for",
            )))
            .response(
                ok(),
                ResponseBuilder::new()
                    .json_content(ProgramSummary::schema().1)
                    .build(),
            )
            .tag(TAG)
            .build();

        let reorder_op = OperationBuilder::new()
            .request_body(Some(
                RequestBodyBuilder::new()
                    .json_content(ReorderSets::schema().1)
                    .build(),
            ))
            .response(
                ok(),
                ResponseBuilder::new()
                    .json_content(Vec::<SetId>::schema())
                    .build(),
            )
            .tag(TAG)
            .build();

        builder
            .path(
                PROGRAMS_PATH,
                PathItemBuilder::new()
                    .operation(PathItemType::Get, get_op)
                    .operation(PathItemType::Post, post_op)
                    .operation(PathItemType::Put, put_op)
                    .build(),
            )
            .path(
                concatcp!(PROGRAMS_PATH, REORDER_SETS_PATH),
                PathItemBuilder::new()
                    .operation(PathItemType::Post, reorder_op)
                    .build(),
            )
            .path(
                concatcp!(PROGRAMS_PATH, "/{id}"),
                PathItemBuilder::new()
                    .operation(PathItemType::Get, summary_op)
                    .operation(PathItemType::Delete, delete_op)
                    .build(),
            )
    }
}
