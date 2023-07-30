use utoipa::{
    openapi::{
        path::{OperationBuilder, PathItemBuilder},
        request_body::RequestBodyBuilder,
        response::Response,
        ComponentsBuilder, Content, PathItemType, PathsBuilder, ResponseBuilder,
    },
    IntoParams, PartialSchema, ToSchema,
};

use crate::{
    openapi::{created, ok, param_in_default, APPLICATION_JSON},
    router::MAXES_PATH,
};

use super::{
    handler::MaxesQuery,
    model::{CreateMax, Max, UpdateMax},
};

pub trait WithMaxesDefinition {
    fn with_maxes(self) -> Self;
}

impl WithMaxesDefinition for ComponentsBuilder {
    fn with_maxes(self) -> Self {
        self.schema_from::<Max>()
            .schema_from::<CreateMax>()
            .schema_from::<UpdateMax>()
    }
}

fn max_response() -> Response {
    ResponseBuilder::new()
        .content(APPLICATION_JSON, Content::new(Max::schema().1))
        .build()
}

const TAG: &str = "Maxes";

impl WithMaxesDefinition for PathsBuilder {
    fn with_maxes(self) -> Self {
        let get_op = OperationBuilder::new()
            .parameters(Some(MaxesQuery::into_params(param_in_default)))
            .response(
                ok(),
                ResponseBuilder::new()
                    .content(APPLICATION_JSON, Content::new(Vec::<Max>::schema()))
                    .build(),
            )
            .tag(TAG)
            .build();

        let post_op = OperationBuilder::new()
            .request_body(Some(
                RequestBodyBuilder::new()
                    .content(APPLICATION_JSON, Content::new(CreateMax::schema().1))
                    .build(),
            ))
            .response(created(), max_response())
            .tag(TAG)
            .build();

        let put_op = OperationBuilder::new()
            .request_body(Some(
                RequestBodyBuilder::new()
                    .content(APPLICATION_JSON, Content::new(UpdateMax::schema().1))
                    .build(),
            ))
            .response(ok(), max_response())
            .tag(TAG)
            .build();

        self.path(
            MAXES_PATH,
            PathItemBuilder::new()
                .operation(PathItemType::Get, get_op)
                .operation(PathItemType::Post, post_op)
                .operation(PathItemType::Put, put_op)
                .build(),
        )
    }
}
