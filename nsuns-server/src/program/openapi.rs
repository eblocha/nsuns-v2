use const_format::concatcp;
use utoipa::{
    openapi::{
        path::{OperationBuilder, PathItemBuilder},
        request_body::RequestBodyBuilder,
        response::Response,
        ComponentsBuilder, Content, PathItemType, PathsBuilder, ResponseBuilder,
    },
    IntoParams, ToSchema, PartialSchema,
};

use crate::{
    openapi::{created, id_path_param, ok, param_in_default, APPLICATION_JSON},
    router::PROGRAMS_PATH,
};

use super::{
    handler::ProgramQuery,
    model::{CreateProgram, ProgramMeta, ProgramSummary, UpdateProgram},
};

pub trait WithProgramsDefintions {
    fn with_programs(self) -> Self;
}

impl WithProgramsDefintions for ComponentsBuilder {
    fn with_programs(self) -> Self {
        self.schema_from::<ProgramMeta>()
            .schema_from::<CreateProgram>()
            .schema_from::<UpdateProgram>()
            .schema_from::<ProgramSummary>()
    }
}

fn program_response() -> Response {
    ResponseBuilder::new()
        .content(APPLICATION_JSON, Content::new(ProgramMeta::schema().1))
        .build()
}

const TAG: &str = "Programs";

impl WithProgramsDefintions for PathsBuilder {
    fn with_programs(self) -> Self {
        let get_op = OperationBuilder::new()
            .parameters(Some(ProgramQuery::into_params(param_in_default)))
            .response(
                ok(),
                ResponseBuilder::new()
                    .content(APPLICATION_JSON, Content::new(Vec::<ProgramMeta>::schema()))
                    .build(),
            )
            .tag(TAG)
            .build();

        let post_op = OperationBuilder::new()
            .request_body(Some(
                RequestBodyBuilder::new()
                    .content(APPLICATION_JSON, Content::new(CreateProgram::schema().1))
                    .build(),
            ))
            .response(created(), program_response())
            .tag(TAG)
            .build();

        let put_op = OperationBuilder::new()
            .request_body(Some(
                RequestBodyBuilder::new()
                    .content(APPLICATION_JSON, Content::new(UpdateProgram::schema().1))
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
                    .content(APPLICATION_JSON, Content::new(ProgramSummary::schema().1))
                    .build(),
            )
            .tag(TAG)
            .build();

        self.path(
            PROGRAMS_PATH,
            PathItemBuilder::new()
                .operation(PathItemType::Get, get_op)
                .operation(PathItemType::Post, post_op)
                .operation(PathItemType::Put, put_op)
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
