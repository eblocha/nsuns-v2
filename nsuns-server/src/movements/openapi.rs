use utoipa::{
    openapi::{
        path::{OperationBuilder, PathItemBuilder},
        request_body::RequestBodyBuilder,
        response::Response,
        ComponentsBuilder, Content, PathItemType, PathsBuilder, ResponseBuilder,
    },
    PartialSchema, ToSchema,
};

use crate::{
    openapi::{created, ok, APPLICATION_JSON},
    router::MOVEMENTS_PATH,
};

use super::model::{CreateMovement, Movement};

pub trait WithMovementsDefinition {
    fn with_movements(self) -> Self;
}

impl WithMovementsDefinition for ComponentsBuilder {
    fn with_movements(self) -> Self {
        self.schema_from::<Movement>()
            .schema_from::<CreateMovement>()
    }
}

fn movement_response() -> Response {
    ResponseBuilder::new()
        .content(APPLICATION_JSON, Content::new(Movement::schema().1))
        .build()
}

const TAG: &str = "Movements";

impl WithMovementsDefinition for PathsBuilder {
    fn with_movements(self) -> Self {
        let get_op = OperationBuilder::new()
            .response(
                ok(),
                ResponseBuilder::new()
                    .content(APPLICATION_JSON, Content::new(Vec::<Movement>::schema()))
                    .build(),
            )
            .tag(TAG)
            .build();

        let post_op = OperationBuilder::new()
            .request_body(Some(
                RequestBodyBuilder::new()
                    .content(APPLICATION_JSON, Content::new(CreateMovement::schema().1))
                    .build(),
            ))
            .response(created(), movement_response())
            .tag(TAG)
            .build();

        let put_op = OperationBuilder::new()
            .request_body(Some(
                RequestBodyBuilder::new()
                    .content(APPLICATION_JSON, Content::new(Movement::schema().1))
                    .build(),
            ))
            .response(ok(), movement_response())
            .tag(TAG)
            .build();

        self.path(
            MOVEMENTS_PATH,
            PathItemBuilder::new()
                .operation(PathItemType::Get, get_op)
                .operation(PathItemType::Post, post_op)
                .operation(PathItemType::Put, put_op)
                .build(),
        )
    }
}
