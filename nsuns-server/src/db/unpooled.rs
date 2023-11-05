use futures_util::TryStreamExt;
use sqlx::{postgres::PgConnectOptions, Connection as SqlxConnection, Database, Executor};

use crate::try_stream;

use super::{Connection, DB};

#[derive(Debug, Clone)]
pub struct UnPooled {
    options: PgConnectOptions,
}

impl UnPooled {
    pub async fn acquire(&self) -> Result<Connection, sqlx::Error> {
        Connection::connect_with(&self.options).await
    }
}

impl From<PgConnectOptions> for UnPooled {
    fn from(options: PgConnectOptions) -> Self {
        Self { options }
    }
}

impl<'c> Executor<'c> for &'c UnPooled {
    type Database = DB;

    fn fetch_many<'e, 'q: 'e, E: 'q>(
        self,
        query: E,
    ) -> futures::stream::BoxStream<
        'e,
        Result<
            sqlx::Either<
                <Self::Database as Database>::QueryResult,
                <Self::Database as Database>::Row,
            >,
            sqlx::Error,
        >,
    >
    where
        'c: 'e,
        E: sqlx::Execute<'q, Self::Database>,
    {
        Box::pin(try_stream! {
            let mut conn = self.acquire().await?;
            let mut s = conn.fetch_many(query);

            while let Some(v) = s.try_next().await? {
                r#yield!(v);
            }

            Ok(())
        })
    }

    fn fetch_optional<'e, 'q: 'e, E: 'q>(
        self,
        query: E,
    ) -> futures::future::BoxFuture<
        'e,
        Result<Option<<Self::Database as Database>::Row>, sqlx::Error>,
    >
    where
        'c: 'e,
        E: sqlx::Execute<'q, Self::Database>,
    {
        Box::pin(async move { self.acquire().await?.fetch_optional(query).await })
    }

    fn prepare_with<'e, 'q: 'e>(
        self,
        sql: &'q str,
        parameters: &'e [<Self::Database as Database>::TypeInfo],
    ) -> futures::future::BoxFuture<
        'e,
        Result<<Self::Database as sqlx::database::HasStatement<'q>>::Statement, sqlx::Error>,
    >
    where
        'c: 'e,
    {
        Box::pin(async move { self.acquire().await?.prepare_with(sql, parameters).await })
    }

    fn describe<'e, 'q: 'e>(
        self,
        sql: &'q str,
    ) -> futures::future::BoxFuture<'e, Result<sqlx::Describe<Self::Database>, sqlx::Error>>
    where
        'c: 'e,
    {
        Box::pin(async move { self.acquire().await?.describe(sql).await })
    }
}
