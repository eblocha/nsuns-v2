use axum::{
    middleware::{from_fn, Next},
    response::IntoResponse,
    Router,
};

use super::extension::HttpRequestAttributes;

async fn collect_attributes<B>(mut request: http::Request<B>, next: Next<B>) -> impl IntoResponse {
    let attrs: HttpRequestAttributes = (&request).into();

    request.extensions_mut().insert(attrs);

    next.run(request).await
}

pub trait CollectAttributes {
    fn collect_attributes(self) -> Self;
}

impl CollectAttributes for Router {
    fn collect_attributes(self) -> Self {
        self.layer(from_fn(collect_attributes))
    }
}
