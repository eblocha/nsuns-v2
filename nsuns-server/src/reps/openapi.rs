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
    openapi::{created, ok, param_in_default, JsonContent},
    router::REPS_PATH,
};

use super::{
    handler::RepsQuery,
    model::{CreateReps, Reps, UpdateReps},
};

pub trait WithRepsDefinition {
    fn with_reps(self) -> Self;
}

impl WithRepsDefinition for ComponentsBuilder {
    fn with_reps(self) -> Self {
        self.schema_from::<Reps>()
            .schema_from::<CreateReps>()
            .schema_from::<UpdateReps>()
    }
}

fn reps_response() -> Response {
    ResponseBuilder::new()
        .json_content(Reps::schema().1)
        .build()
}

const TAG: &str = "Reps";

impl WithRepsDefinition for PathsBuilder {
    fn with_reps(self) -> Self {
        let get_op = OperationBuilder::new()
            .parameters(Some(RepsQuery::into_params(param_in_default)))
            .response(
                ok(),
                ResponseBuilder::new()
                    .json_content(Vec::<Reps>::schema())
                    .build(),
            )
            .tag(TAG)
            .build();

        let post_op = OperationBuilder::new()
            .request_body(Some(
                RequestBodyBuilder::new()
                    .json_content(CreateReps::schema().1)
                    .build(),
            ))
            .response(created(), reps_response())
            .tag(TAG)
            .build();

        let put_op = OperationBuilder::new()
            .request_body(Some(
                RequestBodyBuilder::new()
                    .json_content(CreateReps::schema().1)
                    .build(),
            ))
            .response(ok(), reps_response())
            .tag(TAG)
            .build();

        self.path(
            REPS_PATH,
            PathItemBuilder::new()
                .operation(PathItemType::Get, get_op)
                .operation(PathItemType::Post, post_op)
                .operation(PathItemType::Put, put_op)
                .build(),
        )
    }
}
