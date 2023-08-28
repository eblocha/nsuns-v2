use axum_test_helper::RequestBuilder;
use serde::Serialize;

pub trait JsonBody {
    fn json_body<T>(self, body: &T) -> Self
    where
        T: ?Sized + Serialize;
}

impl JsonBody for RequestBuilder {
    fn json_body<T>(self, body: &T) -> Self
    where
        T: ?Sized + Serialize,
    {
        self.body(serde_json::to_string(body).unwrap())
            .header("Content-Type", "application/json")
    }
}
