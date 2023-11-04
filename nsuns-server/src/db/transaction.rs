use anyhow::Context;
use sqlx::{Acquire, Transaction};
use tracing::Instrument;

use crate::{error::OperationResult, into_log_server_error};

use super::pool::DB;

/// Acquire a new transaction
#[inline]
#[tracing::instrument(name = "begin transaction", skip_all)]
pub async fn transaction<'a>(
    acquire: impl Acquire<'a, Database = DB>,
) -> OperationResult<Transaction<'a, DB>> {
    acquire
        .begin()
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
            .instrument(tracing::info_span!("commit transaction"))
            .await
            .with_context(|| "failed to commit transaction"),
        Err(ref e) => tx
            .rollback()
            .instrument(tracing::info_span!("rollback transaction"))
            .await
            .with_context(|| {
                format!("failed to rollback transaction initiated by previous error: {e:?}")
            }),
    };

    tx_result.map_err(into_log_server_error!()).and(result)
}
