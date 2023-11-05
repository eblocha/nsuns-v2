use futures::{future::BoxFuture, stream::BoxStream, FutureExt, StreamExt};
use opentelemetry_semantic_conventions as semcov;
use sqlx::Executor;
use tracing_futures::Instrument;

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
            // set by executor
            db.statement = tracing::field::Empty,
            // set globally
            db.user = tracing::field::Empty,
            db.connection_string = tracing::field::Empty,
            db.name = tracing::field::Empty,
        )
    };
    ($operation:expr, $table:expr) => {
        tracing::info_span!(
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

pub mod layer {
    use tracing::{field::AsField, span, Subscriber, Value};
    use tracing_subscriber::registry::LookupSpan;

    pub struct GlobalFields<S, F: ?Sized + 'static, V, const N: usize> {
        inner: S,
        pairs: [(&'static F, V); N],
    }

    impl<S, F: ?Sized, V, const N: usize> GlobalFields<S, F, V, N> {
        pub fn new(subscriber: S, pairs: [(&'static F, V); N]) -> Self {
            GlobalFields { inner: subscriber, pairs }
        }
    }

    impl<S: Subscriber, F: ?Sized + AsField + 'static, V: Value + 'static, const N: usize>
        Subscriber for GlobalFields<S, F, V, N>
    {
        fn enabled(&self, metadata: &tracing::Metadata<'_>) -> bool {
            self.inner.enabled(metadata)
        }

        fn new_span(&self, span: &span::Attributes<'_>) -> span::Id {
            let id = self.inner.new_span(span);

            let metadata = span.metadata();

            self.pairs
                .iter()
                .filter_map(|(field, value)| {
                    field
                        .as_field(metadata)
                        .map(|f| (f, Some(value as &dyn Value)))
                })
                .for_each(|(f, v)| {
                    let pair = [(&f, v)];
                    // FIXME this is a hidden API
                    let values = span.fields().value_set(&pair);
                    let values = span::Record::new(&values);

                    self.record(&id, &values);
                });
            id
        }

        fn record(&self, span: &span::Id, values: &span::Record<'_>) {
            self.inner.record(span, values)
        }

        fn record_follows_from(&self, span: &span::Id, follows: &span::Id) {
            self.inner.record_follows_from(span, follows)
        }

        fn event(&self, event: &tracing::Event<'_>) {
            self.inner.event(event)
        }

        fn enter(&self, span: &span::Id) {
            self.inner.enter(span)
        }

        fn exit(&self, span: &span::Id) {
            self.inner.exit(span)
        }

        fn on_register_dispatch(&self, subscriber: &tracing::Dispatch) {
            self.inner.on_register_dispatch(subscriber)
        }

        fn register_callsite(
            &self,
            metadata: &'static tracing::Metadata<'static>,
        ) -> tracing::subscriber::Interest {
            self.inner.register_callsite(metadata)
        }

        fn max_level_hint(&self) -> Option<tracing_subscriber::filter::LevelFilter> {
            self.inner.max_level_hint()
        }

        fn event_enabled(&self, event: &tracing::Event<'_>) -> bool {
            self.inner.event_enabled(event)
        }

        fn clone_span(&self, id: &span::Id) -> span::Id {
            self.inner.clone_span(id)
        }

        fn drop_span(&self, id: span::Id) {
            #[allow(deprecated)]
            self.inner.drop_span(id)
        }

        fn try_close(&self, id: span::Id) -> bool {
            self.inner.try_close(id)
        }

        fn current_span(&self) -> tracing_core::span::Current {
            self.inner.current_span()
        }

        unsafe fn downcast_raw(&self, id: std::any::TypeId) -> Option<*const ()> {
            self.inner.downcast_raw(id)
        }
    }

    impl<
            'span,
            S: LookupSpan<'span>,
            F: ?Sized + AsField + 'static,
            V: Value + 'static,
            const N: usize,
        > LookupSpan<'span> for GlobalFields<S, F, V, N>
    {
        type Data = S::Data;

        fn span_data(&'span self, id: &span::Id) -> Option<Self::Data> {
            self.inner.span_data(id)
        }
    }

    pub trait WithGlobalFields<F: ?Sized, V, const N: usize>
    where
        Self: Sized,
    {
        fn with_global_fields(self, pairs: [(&'static F, V); N]) -> GlobalFields<Self, F, V, N>;
    }

    impl<S, F: ?Sized, V, const N: usize> WithGlobalFields<F, V, N> for S {
        fn with_global_fields(self, pairs: [(&'static F, V); N]) -> GlobalFields<Self, F, V, N> {
            GlobalFields::new(self, pairs)
        }
    }
}
