use anyhow::Context;
use sqlx::{pool::PoolConnection, Acquire, Transaction};
use thiserror::Error;
use tracing::Instrument;

use crate::{db_span, error::OperationResult, into_log_server_error};

use super::{pool::DB, Pool};

/// Acquire a new connection from the pool.
/// Does not convert into OperationResult.
#[inline]
pub async fn acquire_instrumented(pool: &Pool) -> Result<PoolConnection<DB>, sqlx::Error> {
    pool.acquire()
        .instrument(db_span!("acquire connection"))
        .await
}

/// Acquire a new connection from the pool.
/// Useful for creating a separate trace of connect vs. query
#[inline]
pub async fn acquire(pool: &Pool) -> OperationResult<PoolConnection<DB>> {
    acquire_unlogged(pool)
        .await
        .map_err(into_log_server_error!())
}

/// Used for initialization outside of a request context
#[inline]
pub async fn acquire_unlogged(pool: &Pool) -> anyhow::Result<PoolConnection<DB>> {
    acquire_instrumented(pool)
        .await
        .context("failed to acquire connection")
}

/// Acquire a new transaction using an appropriate span
#[inline]
pub async fn transaction_instrumented<'a>(
    acquire: impl Acquire<'a, Database = DB>,
) -> Result<Transaction<'a, DB>, sqlx::Error> {
    acquire
        .begin()
        .instrument(db_span!("begin transaction"))
        .await
}

/// Acquire a new transaction
#[inline]
pub async fn transaction<'a>(
    acquire: impl Acquire<'a, Database = DB>,
) -> OperationResult<Transaction<'a, DB>> {
    transaction_instrumented(acquire)
        .await
        .context("failed to start a transaction")
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
        Ok(_) => commit_instrumented(tx)
            .await
            .context("failed to commit transaction"),
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

#[derive(Debug, Error)]
pub enum CommitError<T, E> {
    /// The transaction failed to commit
    FailedCommit { reason: sqlx::Error, value: T },
    /// The transaction failed to rollback
    FailedRollback {
        reason: sqlx::Error,
        original_error: E,
    },
    /// The transaction rolled back successfully
    SuccessfulRollback(E),
}

/// Commit the transaction inside an appropriate span
#[inline]
pub async fn commit_instrumented(tx: Transaction<'_, DB>) -> Result<(), sqlx::Error> {
    tx.commit().instrument(db_span!("commit transaction")).await
}

/// Commit the transaction if the result is Ok, otherwise rollback.
/// This may transform Ok into Err if the commit fails.
pub async fn commit_ok_instrumented<T, E>(
    result: Result<T, E>,
    tx: Transaction<'_, DB>,
) -> Result<T, CommitError<T, E>> {
    match result {
        Ok(value) => match commit_instrumented(tx).await {
            Ok(_) => Ok(value),
            Err(reason) => Err(CommitError::FailedCommit { reason, value }),
        },
        Err(original_error) => {
            if let Err(reason) = tx
                .rollback()
                .instrument(db_span!("rollback transaction"))
                .await
            {
                return Err(CommitError::FailedRollback {
                    reason,
                    original_error,
                });
            }

            Err(CommitError::SuccessfulRollback(original_error))
        }
    }
}
