use utoipa::{
    openapi::{
        path::{OperationBuilder, PathItemBuilder},
        request_body::RequestBodyBuilder,
        ComponentsBuilder, PathItemType, PathsBuilder, ResponseBuilder,
    },
    ToSchema,
};

use crate::{
    openapi::{ok, JsonContent},
    router::UPDATES_PATH,
};

use super::handler::{DeletedId, Removed, UpdatedState, Updates};

pub trait WithUpdatesDefinition {
    fn with_updates(self) -> Self;
}

impl WithUpdatesDefinition for ComponentsBuilder {
    fn with_updates(self) -> Self {
        self.schema_from::<Updates>()
            .schema_from::<UpdatedState>()
            .schema_from::<DeletedId>()
            .schema_from::<Removed>()
    }
}

const TAG: &str = "Updates";

impl WithUpdatesDefinition for PathsBuilder {
    fn with_updates(self) -> Self {
        let post_op = OperationBuilder::new()
            .request_body(Some(
                RequestBodyBuilder::new()
                    .json_content(Updates::schema().1)
                    .build(),
            ))
            .response(
                ok(),
                ResponseBuilder::new()
                    .json_content(UpdatedState::schema().1)
                    .description("The new maxes and reps as a result of the updates")
                    .build(),
            )
            .tag(TAG)
            .description(Some(
                "Update movements based on the latest reps achieved for each",
            ))
            .build();

        let delete_op = OperationBuilder::new()
            .request_body(Some(
                RequestBodyBuilder::new()
                    .json_content(Updates::schema().1)
                    .build(),
            ))
            .response(
                ok(),
                ResponseBuilder::new()
                    .json_content(Removed::schema().1)
                    .description("The reps and maxes that were removed")
                    .build(),
            )
            .tag(TAG)
            .description(Some(
                "Delete the latest max and reps for each movement specified",
            ))
            .build();

        self.path(
            UPDATES_PATH,
            PathItemBuilder::new()
                .operation(PathItemType::Post, post_op)
                .operation(PathItemType::Delete, delete_op)
                .build(),
        )
    }
}
