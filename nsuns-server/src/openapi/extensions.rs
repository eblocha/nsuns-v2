use axum::http::StatusCode;
use utoipa::openapi::{
    path::{Parameter, ParameterBuilder, ParameterIn},
    request_body::RequestBodyBuilder,
    Content, RefOr, Required, ResponseBuilder, Schema,
};

pub const APPLICATION_JSON: &str = "application/json";

#[must_use]
pub fn ok() -> &'static str {
    StatusCode::OK.as_str()
}

#[must_use]
pub fn created() -> &'static str {
    StatusCode::CREATED.as_str()
}

#[must_use]
pub fn no_content() -> &'static str {
    StatusCode::NO_CONTENT.as_str()
}

#[must_use]
pub fn param_in_default() -> Option<ParameterIn> {
    None
}

#[must_use]
pub fn id_path_param(description: Option<&str>) -> Option<impl IntoIterator<Item = Parameter>> {
    Some([ParameterBuilder::new()
        .name("id")
        .description(description)
        .required(Required::True)
        .parameter_in(ParameterIn::Path)
        .build()])
}

pub trait JsonContent {
    #[must_use]
    fn json_content<S>(self, schema: S) -> Self
    where
        S: Into<RefOr<Schema>>;
}

impl JsonContent for ResponseBuilder {
    fn json_content<S>(self, schema: S) -> Self
    where
        S: Into<RefOr<Schema>>,
    {
        self.content(APPLICATION_JSON, Content::new(schema))
    }
}

impl JsonContent for RequestBodyBuilder {
    fn json_content<S>(self, schema: S) -> Self
    where
        S: Into<RefOr<Schema>>,
    {
        self.content(APPLICATION_JSON, Content::new(schema))
    }
}
