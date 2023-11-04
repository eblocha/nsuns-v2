/// Creates a tracing span for calling out to the database
#[macro_export]
macro_rules! db_span {
    () => {
        db_span!("database query")
    };
    ($name:expr) => {
        tracing::info_span!(
            $name,
            otel.kind = ?opentelemetry_api::trace::SpanKind::Client,
            db.system = $crate::db::pool::DB_NAME,
            db.statement = tracing::field::Empty
        )
    };
}
