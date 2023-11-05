use futures::{future::BoxFuture, stream::BoxStream, FutureExt, StreamExt};
use opentelemetry_semantic_conventions as semcov;
use sqlx::Executor;
use tracing_futures::Instrument;

// Creates a span/event target with a suffix to indicate is a database query span
#[macro_export]
macro_rules! db_target {
    () => {
        const_format::concatcp!(module_path!(), $crate::db::tracing::TRACING_TARGET_SUFFIX)
    };
}

/// Creates a tracing span for calling out to the database
#[macro_export]
macro_rules! db_span {
    () => {
        db_span!("database query")
    };
    ($name:expr) => {
        tracing::info_span!(
            target: $crate::db_target!(),
            $name,
            otel.kind = ?opentelemetry_api::trace::SpanKind::Client,
            db.system = $crate::db::pool::DB_NAME,
            // set by executor
            db.statement = tracing::field::Empty,
            // set globally
            db.user = tracing::field::Empty,
            db.connection_string = tracing::field::Empty,
            db.name = tracing::field::Empty,
            server.address = tracing::field::Empty,
        )
    };
    ($operation:expr, $table:expr) => {
        tracing::info_span!(
            target: $crate::db_target!(),
            const_format::concatcp!($operation, " ", $table),
            otel.kind = ?opentelemetry_api::trace::SpanKind::Client,
            db.system = $crate::db::pool::DB_NAME,
            db.operation = $operation,
            db.sql.table = $table,
            // set by executor
            db.statement = tracing::field::Empty,
            // set globally
            db.user = tracing::field::Empty,
            db.connection_string = tracing::field::Empty,
            db.name = tracing::field::Empty,
            server.address = tracing::field::Empty,
        )
    }
}

#[derive(Debug)]
pub struct InstrumentedExecutor<Executor> {
    inner: Executor,
    span: tracing::Span,
}

impl<Ex> InstrumentedExecutor<Ex> {
    fn record_sql(&self, sql: &str) {
        self.span.record(semcov::trace::DB_STATEMENT.as_str(), sql);
    }
}

pub trait InstrumentExecutor
where
    Self: Sized,
{
    /// Instrument an executor with a span to record database statements and enter the span when executing them.
    fn instrument_executor(self, span: tracing::Span) -> InstrumentedExecutor<Self> {
        InstrumentedExecutor { inner: self, span }
    }
}

impl<T> InstrumentExecutor for T {}

impl<'c, Ex> Executor<'c> for InstrumentedExecutor<Ex>
where
    Ex: Executor<'c>,
{
    type Database = Ex::Database;

    fn fetch_many<'e, 'q: 'e, E: 'q>(
        self,
        query: E,
    ) -> BoxStream<
        'e,
        Result<
            sqlx::Either<
                <Self::Database as sqlx::Database>::QueryResult,
                <Self::Database as sqlx::Database>::Row,
            >,
            sqlx::Error,
        >,
    >
    where
        'c: 'e,
        E: sqlx::Execute<'q, Self::Database>,
    {
        self.record_sql(query.sql());
        self.inner.fetch_many(query).instrument(self.span).boxed()
    }

    fn fetch_optional<'e, 'q: 'e, E: 'q>(
        self,
        query: E,
    ) -> BoxFuture<'e, Result<Option<<Self::Database as sqlx::Database>::Row>, sqlx::Error>>
    where
        'c: 'e,
        E: sqlx::Execute<'q, Self::Database>,
    {
        self.record_sql(query.sql());
        self.inner
            .fetch_optional(query)
            .instrument(self.span)
            .boxed()
    }

    fn prepare_with<'e, 'q: 'e>(
        self,
        sql: &'q str,
        parameters: &'e [<Self::Database as sqlx::Database>::TypeInfo],
    ) -> BoxFuture<
        'e,
        Result<<Self::Database as sqlx::database::HasStatement<'q>>::Statement, sqlx::Error>,
    >
    where
        'c: 'e,
    {
        self.record_sql(sql);
        self.inner
            .prepare_with(sql, parameters)
            .instrument(self.span)
            .boxed()
    }

    fn describe<'e, 'q: 'e>(
        self,
        sql: &'q str,
    ) -> BoxFuture<'e, Result<sqlx::Describe<Self::Database>, sqlx::Error>>
    where
        'c: 'e,
    {
        self.inner.describe(sql)
    }

    // DEFAULT MEMBERS (in case the inner Executor overrides these)

    fn execute<'e, 'q: 'e, E: 'q>(
        self,
        query: E,
    ) -> BoxFuture<'e, Result<<Self::Database as sqlx::Database>::QueryResult, sqlx::Error>>
    where
        'c: 'e,
        E: sqlx::Execute<'q, Self::Database>,
    {
        self.record_sql(query.sql());
        self.inner.execute(query).instrument(self.span).boxed()
    }

    fn execute_many<'e, 'q: 'e, E: 'q>(
        self,
        query: E,
    ) -> BoxStream<'e, Result<<Self::Database as sqlx::Database>::QueryResult, sqlx::Error>>
    where
        'c: 'e,
        E: sqlx::Execute<'q, Self::Database>,
    {
        self.record_sql(query.sql());
        self.inner.execute_many(query).instrument(self.span).boxed()
    }

    fn fetch<'e, 'q: 'e, E: 'q>(
        self,
        query: E,
    ) -> BoxStream<'e, Result<<Self::Database as sqlx::Database>::Row, sqlx::Error>>
    where
        'c: 'e,
        E: sqlx::Execute<'q, Self::Database>,
    {
        self.record_sql(query.sql());
        self.inner.fetch(query).instrument(self.span).boxed()
    }

    fn fetch_all<'e, 'q: 'e, E: 'q>(
        self,
        query: E,
    ) -> BoxFuture<'e, Result<Vec<<Self::Database as sqlx::Database>::Row>, sqlx::Error>>
    where
        'c: 'e,
        E: sqlx::Execute<'q, Self::Database>,
    {
        self.record_sql(query.sql());
        self.inner.fetch_all(query).instrument(self.span).boxed()
    }

    fn fetch_one<'e, 'q: 'e, E: 'q>(
        self,
        query: E,
    ) -> BoxFuture<'e, Result<<Self::Database as sqlx::Database>::Row, sqlx::Error>>
    where
        'c: 'e,
        E: sqlx::Execute<'q, Self::Database>,
    {
        self.record_sql(query.sql());
        self.inner.fetch_one(query).instrument(self.span).boxed()
    }

    fn prepare<'e, 'q: 'e>(
        self,
        query: &'q str,
    ) -> BoxFuture<
        'e,
        Result<<Self::Database as sqlx::database::HasStatement<'q>>::Statement, sqlx::Error>,
    >
    where
        'c: 'e,
    {
        self.record_sql(query);
        self.inner.prepare(query).instrument(self.span).boxed()
    }
}

pub mod statements {
    pub const SELECT: &str = "SELECT";
    pub const UPDATE: &str = "UPDATE";
    pub const INSERT_INTO: &str = "INSERT INTO";
    pub const DELETE_FROM: &str = "DELETE FROM";
}

pub const TRACING_TARGET_SUFFIX: &str = "/db::tracing";
