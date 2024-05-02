use anyhow::Context;
use sqlx::Transaction;
use thiserror::Error;
use tracing::Instrument;

use crate::{db_span, error::OperationResult, into_log_server_error};

use super::pool::DB;

/// Create a span for interacting with the database pool, such as acquiring connections.
///
/// This will record information about the pool in the span.
#[macro_export]
macro_rules! pool_span {
    ($name:expr, $pool:expr) => {
        tracing::info_span!(
            $name,
            otel.kind = ?opentelemetry_api::trace::SpanKind::Client,
            db.system = $crate::db::pool::DB_NAME,
            db.statement = tracing::field::Empty,
            db.user = $pool.settings.username,
            db.name = $pool.settings.database,
            server.address = $pool.settings.host,
            server.port = $pool.settings.port
        )
    };
}

/// Acquire an instrumented connection from the connection pool
#[macro_export]
macro_rules! acquire {
    ($pool:expr) => {
        async {
            $crate::acquire_unlogged!(&$pool)
                .await
                .map_err($crate::into_log_server_error!())
        }
    };
}

/// Acquire an instrumented connection from the connection pool, without logging the error
#[macro_export]
macro_rules! acquire_unlogged {
    ($pool:expr) => {
        async {
            anyhow::Context::context(
                tracing_futures::Instrument::instrument(
                    sqlx::Acquire::acquire(&$pool.inner),
                    $crate::pool_span!("acquire connection", $pool),
                )
                .await,
                "failed to acquire connection",
            )
        }
    };
}

/// Begin a transaction. This will enter an appropriate span while beginning the transaction
#[macro_export]
macro_rules! transaction {
    ($pool:expr) => {
        async {
            anyhow::Context::context(
                tracing_futures::Instrument::instrument(
                    sqlx::Transaction::begin($crate::acquire!($pool).await?),
                    $crate::pool_span!("begin transaction", $pool),
                )
                .await,
                "failed to begin transaction",
            )
            .map_err($crate::into_log_server_error!())
        }
    };
}

/// Create an instrumented executor from a connection pool.
///
/// Note that the executor's span will be very generic, so prefer to create an executor with
/// `acquire!(&pool)` or `transaction!(&pool)`, and use `InstrumentedExecutor` to add a more specific span.
#[macro_export]
macro_rules! as_executor {
    ($pool:expr) => {
        $crate::db::tracing::InstrumentExecutor::instrument_executor(
            &$pool.inner,
            $crate::pool_span!("database query", $pool),
        )
    };
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
            Ok(()) => Ok(value),
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
