use std::fmt::Display;

use anyhow::{anyhow, Context};
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use sqlx::{Executor, Transaction};
use tracing::Instrument;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::{
    db::DB,
    error::{ErrorWithStatus, OperationResult},
    into_log_server_error, log_server_error,
    program::model::{get_set_ids, update_set_ids}, db_span,
};

fn handle_error<F, C>(e: sqlx::Error, context: F) -> ErrorWithStatus<anyhow::Error>
where
    F: FnOnce() -> C,
    C: Display + Send + Sync + 'static,
{
    match e {
        sqlx::Error::Database(e) if e.is_foreign_key_violation() => ErrorWithStatus::new(
            StatusCode::BAD_REQUEST,
            anyhow!("programId, movementId, or percentageOfMax provided does not exist"),
        ),
        _ => anyhow!(e).context(context()).into(),
    }
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone, Copy, sqlx::Type)]
#[repr(i16)]
pub enum Day {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow, ToSchema, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Set {
    pub id: Uuid,
    pub program_id: Uuid,
    #[schema(value_type = i16)]
    pub day: Day,
    pub movement_id: Uuid,
    pub reps: Option<i32>,
    pub reps_is_minimum: bool,
    pub description: Option<String>,
    pub amount: f64,
    pub percentage_of_max: Option<Uuid>,
}

impl Set {
    #[tracing::instrument(name = "Set::select_where_id_in", skip_all)]
    pub async fn select_where_id_in(
        ids: &[Uuid],
        executor: impl Executor<'_, Database = DB>,
    ) -> Result<Vec<Set>, sqlx::Error> {
        sqlx::query_as::<_, Set>(
            "SELECT * FROM program_sets WHERE id = any($1) ORDER BY array_position($2, id)",
        )
        .bind(ids)
        .bind(ids)
        .fetch_all(executor)
        .instrument(db_span!())
        .await
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateSet {
    pub program_id: Uuid,
    pub movement_id: Uuid,
    #[schema(value_type = i16)]
    pub day: Day,
    #[validate(range(min = 0))]
    pub reps: Option<i32>,
    #[serde(default)]
    pub reps_is_minimum: bool,
    #[validate(length(min = 1))]
    pub description: Option<String>,
    #[validate(range(min = 0))]
    pub amount: f64,
    pub percentage_of_max: Option<Uuid>,
}

impl CreateSet {
    #[tracing::instrument(name = "CreateSet::insert_one", skip_all)]
    pub async fn insert_one(self, tx: &mut Transaction<'_, DB>) -> OperationResult<Option<Set>> {
        let set_ids = get_set_ids(self.program_id, self.day, true, &mut **tx).await?;

        if let Some(mut set_ids) = set_ids {
            let id = sqlx::query_as::<_, (Uuid,)>(
                "INSERT INTO program_sets (
                    movement_id, reps, reps_is_minimum, description, amount, percentage_of_max, program_id, day
                ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING id",
            )
            .bind(self.movement_id)
            .bind(self.reps)
            .bind(self.reps_is_minimum)
            .bind(&self.description)
            .bind(self.amount)
            .bind(self.percentage_of_max)
            .bind(self.program_id)
            .bind(self.day)
            .fetch_one(&mut **tx)
            .instrument(db_span!())
            .await
            .map_err(|e| handle_error(e, || "failed to insert new set"))
            .map_err(log_server_error!())?
            .0;

            set_ids.push(id);

            update_set_ids(self.program_id, self.day, &set_ids, &mut **tx).await?;

            Ok(Some(Set {
                id,
                program_id: self.program_id,
                day: self.day,
                movement_id: self.movement_id,
                reps: self.reps,
                reps_is_minimum: self.reps_is_minimum,
                description: self.description,
                amount: self.amount,
                percentage_of_max: self.percentage_of_max,
            }))
        } else {
            Ok(None)
        }
    }
}

#[tracing::instrument(name = "Set::delete_one", skip(tx))]
pub async fn delete_one(id: Uuid, tx: &mut Transaction<'_, DB>) -> OperationResult<Option<()>> {
    let res = sqlx::query_as::<_, (Uuid, Day)>(
        "DELETE FROM program_sets WHERE id = $1 RETURNING program_id, day",
    )
    .bind(id)
    .fetch_optional(&mut **tx)
    .instrument(db_span!())
    .await
    .with_context(|| format!("failed to delete set with id={id}"))
    .map_err(into_log_server_error!())?;

    if let Some((program_id, day)) = res {
        let set_ids = get_set_ids(program_id, day, true, &mut **tx).await?;

        if let Some(set_ids) = set_ids {
            let set_ids = set_ids.into_iter().filter(|set_id| *set_id != id).collect();
            update_set_ids(program_id, day, &set_ids, &mut **tx).await?;
        }
        Ok(Some(()))
    } else {
        Ok(None)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSet {
    pub id: Uuid,
    pub movement_id: Uuid,
    #[validate(range(min = 0))]
    pub reps: Option<i32>,
    #[serde(default)]
    pub reps_is_minimum: bool,
    #[validate(length(min = 1))]
    pub description: Option<String>,
    #[validate(range(min = 0))]
    pub amount: f64,
    pub percentage_of_max: Option<Uuid>,
}

impl UpdateSet {
    #[tracing::instrument(name = "UpdateSet::update_one", skip_all)]
    pub async fn update_one(
        self,
        executor: impl Executor<'_, Database = DB>,
    ) -> OperationResult<Option<Set>> {
        let res = sqlx::query_as::<_, (Uuid, Day)>(
            "UPDATE program_sets SET
            movement_id = $1,
            reps = $2,
            reps_is_minimum = $3,
            description = $4,
            amount = $5,
            percentage_of_max = $6
            WHERE id = $7
            RETURNING program_id, day
        ",
        )
        .bind(self.movement_id)
        .bind(self.reps)
        .bind(self.reps_is_minimum)
        .bind(self.description.as_ref())
        .bind(self.amount)
        .bind(self.percentage_of_max)
        .bind(self.id)
        .fetch_optional(executor)
        .instrument(db_span!())
        .await
        .map_err(|e| {
            handle_error(e, || {
                format!("failed to update set with id={id}", id = self.id)
            })
        })
        .map_err(log_server_error!())?;

        Ok(res.map(|(program_id, day)| Set {
            id: self.id,
            program_id,
            day,
            movement_id: self.movement_id,
            reps: self.reps,
            reps_is_minimum: self.reps_is_minimum,
            description: self.description,
            amount: self.amount,
            percentage_of_max: self.percentage_of_max,
        }))
    }
}
