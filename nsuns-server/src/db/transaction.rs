use anyhow::Context;
use sqlx::{pool::PoolConnection, Acquire, Transaction};
use tracing::Instrument;

use crate::{db_span, error::OperationResult, into_log_server_error};

use super::{pool::DB, Pool};

/// Acquire a new connection from the pool.
/// Useful for creating a separate trace of connect vs. query
pub async fn acquire(pool: &Pool) -> OperationResult<PoolConnection<DB>> {
    acquire_unlogged(pool)
        .await
        .map_err(into_log_server_error!())
}

/// Used for initialization outside of a request context
pub async fn acquire_unlogged(pool: &Pool) -> anyhow::Result<PoolConnection<DB>> {
    pool.acquire()
        .instrument(db_span!("acquire connection"))
        .await
        .context("failed to acquire connection")
}

/// Acquire a new transaction
#[inline]
pub async fn transaction<'a>(
    acquire: impl Acquire<'a, Database = DB>,
) -> OperationResult<Transaction<'a, DB>> {
    acquire
        .begin()
        .instrument(db_span!("begin transaction"))
        .await
        .with_context(|| "failed to start a transaction")
        .map_err(into_log_server_error!())
}

/// Commit the transaction if the result is Ok, otherwise rollback.
/// This may transform Ok into Err if the commit fails.
#[inline]
pub async fn commit_ok<T>(
    result: OperationResult<T>,
    tx: Transaction<'_, DB>,
) -> OperationResult<T> {
    let tx_result = match result {
        Ok(_) => tx
            .commit()
            .instrument(db_span!("commit transaction"))
            .await
            .with_context(|| "failed to commit transaction"),
        Err(ref e) => tx
            .rollback()
            .instrument(db_span!("rollback transaction"))
            .await
            .with_context(|| {
                format!("failed to rollback transaction initiated by previous error: {e:?}")
            }),
    };

    tx_result.map_err(into_log_server_error!()).and(result)
}
