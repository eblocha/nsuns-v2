use sqlx::{Executor, Database, Acquire};

use super::{DB, unpooled::UnPooled, Connection, PoolConnection};

#[derive(Debug, Clone)]
pub enum MaybePool {
    Pool(sqlx::Pool<DB>),
    OnDemand(UnPooled),
}

impl<'c> Executor<'c> for &'c MaybePool {
    type Database = DB;

    fn fetch_many<'e, 'q: 'e, E: 'q>(
        self,
        query: E,
    ) -> futures_core::stream::BoxStream<
        'e,
        Result<
            sqlx_core::Either<
                <Self::Database as Database>::QueryResult,
                <Self::Database as Database>::Row,
            >,
            sqlx_core::Error,
        >,
    >
    where
        'c: 'e,
        E: sqlx::Execute<'q, Self::Database>,
    {
        match self {
            MaybePool::Pool(p) => p.fetch_many(query),
            MaybePool::OnDemand(c) => c.fetch_many(query),
        }
    }

    fn fetch_optional<'e, 'q: 'e, E: 'q>(
        self,
        query: E,
    ) -> futures_core::future::BoxFuture<
        'e,
        Result<Option<<Self::Database as Database>::Row>, sqlx_core::Error>,
    >
    where
        'c: 'e,
        E: sqlx::Execute<'q, Self::Database>,
    {
        match self {
            MaybePool::Pool(p) => p.fetch_optional(query),
            MaybePool::OnDemand(c) => c.fetch_optional(query),
        }
    }

    fn prepare_with<'e, 'q: 'e>(
        self,
        sql: &'q str,
        parameters: &'e [<Self::Database as Database>::TypeInfo],
    ) -> futures_core::future::BoxFuture<
        'e,
        Result<
            <Self::Database as sqlx_core::database::HasStatement<'q>>::Statement,
            sqlx_core::Error,
        >,
    >
    where
        'c: 'e,
    {
        match self {
            MaybePool::Pool(p) => p.prepare_with(sql, parameters),
            MaybePool::OnDemand(c) => c.prepare_with(sql, parameters),
        }
    }

    fn describe<'e, 'q: 'e>(
        self,
        sql: &'q str,
    ) -> futures_core::future::BoxFuture<'e, Result<sqlx::Describe<Self::Database>, sqlx_core::Error>>
    where
        'c: 'e,
    {
        match self {
            MaybePool::Pool(p) => p.describe(sql),
            MaybePool::OnDemand(c) => c.describe(sql),
        }
    }
}

impl<'c> Acquire<'c> for &'c mut MaybePooledConnection {
    type Database = DB;

    type Connection = &'c mut Connection;

    fn acquire(
        self,
    ) -> futures_core::future::BoxFuture<'c, Result<Self::Connection, sqlx_core::Error>> {
        match self {
            MaybePooledConnection::Pooled(p) => p.acquire(),
            MaybePooledConnection::OnDemand(c) => c.acquire(),
        }
    }

    fn begin(
        self,
    ) -> futures_core::future::BoxFuture<
        'c,
        Result<sqlx::Transaction<'c, Self::Database>, sqlx_core::Error>,
    > {
        match self {
            MaybePooledConnection::Pooled(p) => p.begin(),
            MaybePooledConnection::OnDemand(c) => c.begin(),
        }
    }
}

pub enum MaybePooledConnection {
    Pooled(PoolConnection),
    OnDemand(Connection),
}