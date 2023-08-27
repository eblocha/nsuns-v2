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
    openapi::{created, ok, JsonContent},
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
        .json_content(Movement::schema().1)
        .build()
}

const TAG: &str = "Movements";

impl WithMovementsDefinition for PathsBuilder {
    fn with_movements(self) -> Self {
        let get_op = OperationBuilder::new()
            .response(
                ok(),
                ResponseBuilder::new()
                    .json_content(Vec::<Movement>::schema())
                    .build(),
            )
            .tag(TAG)
            .build();

        let post_op = OperationBuilder::new()
            .request_body(Some(
                RequestBodyBuilder::new()
                    .json_content(CreateMovement::schema().1)
                    .build(),
            ))
            .response(created(), movement_response())
            .tag(TAG)
            .build();

        let put_op = OperationBuilder::new()
            .request_body(Some(
                RequestBodyBuilder::new()
                    .json_content(Movement::schema().1)
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
