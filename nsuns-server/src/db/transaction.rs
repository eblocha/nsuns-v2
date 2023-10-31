use std::fmt::Debug;

use anyhow::Context;
use sqlx::{Acquire, Transaction};
use tracing::Instrument;

use crate::error::{ErrorWithStatus, OperationResult};

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
        .map_err(Into::into)
}

/// Commit the transaction if the result is Ok, otherwise rollback.
/// This may transform Ok into Err if the commit fails.
#[inline]
pub async fn commit_ok<T, E>(result: Result<T, E>, tx: Transaction<'_, DB>) -> OperationResult<T>
where
    E: Into<ErrorWithStatus<anyhow::Error>> + Debug,
{
    match result {
        Ok(_) => {
            tx.commit()
                .instrument(tracing::info_span!("commit transaction"))
                .await
                .with_context(|| "failed to commit transaction")?;
        }
        Err(ref e) => {
            tx.rollback()
                .instrument(tracing::info_span!("rollback transaction"))
                .await
                .with_context(|| {
                    format!("failed to rollback transaction initiated by previous error: {e:?}")
                })?;
        }
    };

    result.map_err(Into::into)
}
