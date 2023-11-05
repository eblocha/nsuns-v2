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
    router::MAXES_PATH,
};

use super::{
    handler::MaxesQuery,
    model::{CreateMax, Max, UpdateMax},
};

pub struct MaxesModule;

fn max_response() -> Response {
    ResponseBuilder::new().json_content(Max::schema().1).build()
}

const TAG: &str = "Maxes";

impl Customizer<ComponentsBuilder> for MaxesModule {
    fn customize(builder: ComponentsBuilder) -> ComponentsBuilder {
        builder
            .schema_from::<Max>()
            .schema_from::<CreateMax>()
            .schema_from::<UpdateMax>()
    }
}

impl Customizer<PathsBuilder> for MaxesModule {
    fn customize(builder: PathsBuilder) -> PathsBuilder {
        let get_op = OperationBuilder::new()
            .parameters(Some(MaxesQuery::into_params(param_in_default)))
            .response(
                ok(),
                ResponseBuilder::new()
                    .json_content(Vec::<Max>::schema())
                    .build(),
            )
            .tag(TAG)
            .build();

        let post_op = OperationBuilder::new()
            .request_body(Some(
                RequestBodyBuilder::new()
                    .json_content(CreateMax::schema().1)
                    .build(),
            ))
            .response(created(), max_response())
            .tag(TAG)
            .build();

        let put_op = OperationBuilder::new()
            .request_body(Some(
                RequestBodyBuilder::new()
                    .json_content(UpdateMax::schema().1)
                    .build(),
            ))
            .response(ok(), max_response())
            .tag(TAG)
            .build();

        builder.path(
            MAXES_PATH,
            PathItemBuilder::new()
                .operation(PathItemType::Get, get_op)
                .operation(PathItemType::Post, post_op)
                .operation(PathItemType::Put, put_op)
                .build(),
        )
    }
}
