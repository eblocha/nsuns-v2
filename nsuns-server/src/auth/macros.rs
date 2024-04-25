/// Generates an expression that will execute a DB query to validate a row is owned by owner_id.
///
/// The row must use `id` as the column name of the row's primary key.
///
/// The expression evaluates to `OperationResult<()>`, with a status code of 409 (conflict) if the row does not match
/// or does not exist.
#[macro_export]
macro_rules! assert_owner {
    ($table:expr, $entity_name:expr, $id:ident, $owner_id:ident, $executor:ident) => {
        sqlx::query_as::<_, (bool,)>(const_format::concatcp!(
            "SELECT EXISTS(SELECT 1 FROM ",
            $table,
            " WHERE id = $1 AND owner_id = $2)"
        ))
        .bind($id)
        .bind($owner_id)
        .fetch_one($executor.instrument_executor($crate::db_span!(
            $crate::db::tracing::statements::SELECT,
            $table
        )))
        .await
        .with_context(|| format!("failed to select {} with id={}", $entity_name, $id))
        .map_err($crate::into_log_server_error!())
        .and_then(|(exists,)| {
            if exists {
                Ok(())
            } else {
                Err($crate::error::ErrorWithStatus::new(
                    http::StatusCode::CONFLICT,
                    anyhow::anyhow!(const_format::concatcp!(
                        "referenced ",
                        $entity_name,
                        " does not exist"
                    )),
                ))
            }
        })
    };
}
