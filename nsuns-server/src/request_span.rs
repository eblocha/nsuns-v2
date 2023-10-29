use tower_http::trace::MakeSpan;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct RequestSpan;

impl<B> MakeSpan<B> for RequestSpan {
    fn make_span(&mut self, request: &hyper::Request<B>) -> tracing::Span {
        tracing::info_span!(
            "request",
            method = %request.method(),
            uri = %request.uri(),
            request_id = %Uuid::new_v4(),
            version = ?request.version(),
        )
    }
}
