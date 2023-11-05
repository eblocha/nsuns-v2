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
        extensions::{created, ok, param_in_default, JsonContent},
        Customizer,
    },
    router::REPS_PATH,
};

use super::{
    handler::RepsQuery,
    model::{CreateReps, Reps, UpdateReps},
};

pub struct RepsModule;

fn reps_response() -> Response {
    ResponseBuilder::new()
        .json_content(Reps::schema().1)
        .build()
}

const TAG: &str = "Reps";

impl Customizer<ComponentsBuilder> for RepsModule {
    fn customize(builder: ComponentsBuilder) -> ComponentsBuilder {
        builder
            .schema_from::<Reps>()
            .schema_from::<CreateReps>()
            .schema_from::<UpdateReps>()
    }
}

impl Customizer<PathsBuilder> for RepsModule {
    fn customize(builder: PathsBuilder) -> PathsBuilder {
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

        builder.path(
            REPS_PATH,
            PathItemBuilder::new()
                .operation(PathItemType::Get, get_op)
                .operation(PathItemType::Post, post_op)
                .operation(PathItemType::Put, put_op)
                .build(),
        )
    }
}
