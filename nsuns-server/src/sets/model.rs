use std::fmt::Display;

use anyhow::{anyhow, Context};
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::{Executor, Transaction};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::{
    db::DB,
    error::{ErrorWithStatus, OperationResult},
};

fn handle_error<F, C>(e: sqlx::Error, context: F) -> ErrorWithStatus<anyhow::Error>
where
    F: FnOnce() -> C,
    C: Display + Send + Sync + 'static,
{
    match e {
        sqlx::Error::Database(e) if e.is_foreign_key_violation() => ErrorWithStatus {
            status: StatusCode::BAD_REQUEST,
            error: anyhow!("programId, movementId, or percentageOfMax provided does not exist"),
        },
        _ => anyhow!(e).context(context()).into(),
    }
}

#[derive(Debug, Serialize, Clone, sqlx::FromRow, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Set {
    pub id: Uuid,
    pub program_id: Uuid,
    pub movement_id: Uuid,
    pub day: i16,
    pub ordering: i32,
    pub reps: Option<i32>,
    pub reps_is_minimum: bool,
    pub description: Option<String>,
    pub amount: f64,
    pub percentage_of_max: Option<Uuid>,
}

impl Set {
    pub async fn select_for_program(
        program_id: Uuid,
        executor: impl Executor<'_, Database = DB>,
    ) -> OperationResult<Vec<Set>> {
        sqlx::query_as::<_, Self>(
            "SELECT * FROM program_sets WHERE program_id = $1 ORDER BY day, ordering",
        )
        .bind(program_id)
        .fetch_all(executor)
        .await
        .with_context(|| format!("failed to select sets with program_id={program_id}"))
        .map_err(Into::into)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateSet {
    pub program_id: Uuid,
    pub movement_id: Uuid,
    #[validate(range(min = 0, max = 6))]
    pub day: i16,
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
    pub async fn insert_one(self, tx: &mut Transaction<'_, DB>) -> OperationResult<Set> {
        // get the max `ordering` value for the program and day
        // fetching all rows here to lock them for updates
        let ordering = sqlx::query_as::<_, (i32,)>(
            "SELECT ordering FROM program_sets WHERE program_id = $1 AND day = $2 FOR UPDATE",
        )
        .bind(self.program_id)
        .bind(self.day)
        .fetch_all(&mut **tx)
        .await
        .with_context(|| {
            format!(
                "failed to fetch max ordered set with program_id={} and day={}",
                self.program_id, self.day
            )
        })?
        .into_iter()
        .map(|r| r.0 + 1)
        .max()
        .unwrap_or(0);

        let id = sqlx::query_as::<_, (Uuid,)>(
            "INSERT INTO program_sets (
            program_id, movement_id, day, ordering, reps, reps_is_minimum, description, amount, percentage_of_max
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING id",
        )
        .bind(self.program_id)
        .bind(self.movement_id)
        .bind(self.day)
        .bind(ordering)
        .bind(self.reps)
        .bind(self.reps_is_minimum)
        .bind(&self.description)
        .bind(self.amount)
        .bind(self.percentage_of_max)
        .fetch_one(&mut **tx)
        .await
        .map_err(|e| handle_error(e, || "failed to insert new set"))?
        .0;

        Ok(Set {
            id,
            program_id: self.program_id,
            movement_id: self.movement_id,
            day: self.day,
            ordering,
            reps: self.reps,
            reps_is_minimum: self.reps_is_minimum,
            description: self.description,
            amount: self.amount,
            percentage_of_max: self.percentage_of_max,
        })
    }
}

pub async fn delete_one(id: Uuid, tx: &mut Transaction<'_, DB>) -> OperationResult<Option<Set>> {
    tx.execute("LOCK TABLE program_sets IN ACCESS EXCLUSIVE MODE")
        .await
        .with_context(|| "failed to lock program_sets table")?;

    let set_opt = sqlx::query_as::<_, Set>("DELETE FROM program_sets WHERE id = $1 RETURNING *")
        .bind(id)
        .fetch_optional(&mut **tx)
        .await
        .with_context(|| format!("failed to delete set with id={id}"))?;

    if let Some(ref set) = set_opt {
        // decrement any sets with ordering > this one
        sqlx::query("UPDATE program_sets SET ordering = ordering - 1 WHERE program_id = $2 AND day = $3 AND ordering > $1")
            .bind(set.ordering)
            .bind(set.program_id)
            .bind(set.day)
            .execute(&mut **tx)
            .await
            .with_context(|| "failed to decrement remaining set ordering")?;
    }

    Ok(set_opt)
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSet {
    pub id: Uuid,
    pub program_id: Uuid,
    pub movement_id: Uuid,
    #[validate(range(min = 0, max = 6))]
    pub day: i16,
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
    pub async fn update_one(
        self,
        executor: impl Executor<'_, Database = DB>,
    ) -> OperationResult<Option<Set>> {
        let opt = sqlx::query_as::<_, (Uuid, i32)>(
            "UPDATE program_sets SET
            program_id = $1,
            movement_id = $2,
            day = $3,
            reps = $4,
            reps_is_minimum = $5,
            description = $6,
            amount = $7,
            percentage_of_max = $8
            WHERE id = $9
            RETURNING id, ordering
        ",
        )
        .bind(self.program_id)
        .bind(self.movement_id)
        .bind(self.day)
        .bind(self.reps)
        .bind(self.reps_is_minimum)
        .bind(self.description.as_ref())
        .bind(self.amount)
        .bind(self.percentage_of_max)
        .bind(self.id)
        .fetch_optional(executor)
        .await
        .map_err(|e| {
            handle_error(e, || {
                format!("failed to update set with id={id}", id = self.id)
            })
        })?;

        if let Some((id, ordering)) = opt {
            Ok(Some(Set {
                id,
                program_id: self.program_id,
                movement_id: self.movement_id,
                day: self.day,
                ordering,
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
