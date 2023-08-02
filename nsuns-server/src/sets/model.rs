use std::fmt::Display;

use anyhow::{anyhow, Context};
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use sqlx::{postgres::PgQueryResult, Executor, Transaction};
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

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone, Copy)]
#[repr(u8)]
pub enum Day {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

#[derive(Debug, Serialize, Clone, sqlx::FromRow, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Set {
    pub id: Uuid,
    pub movement_id: Uuid,
    pub reps: Option<i32>,
    pub reps_is_minimum: bool,
    pub description: Option<String>,
    pub amount: f64,
    pub percentage_of_max: Option<Uuid>,
}

impl Set {
    pub async fn select_where_id_in(
        ids: &Vec<Uuid>,
        executor: impl Executor<'_, Database = DB>,
    ) -> Result<Vec<Set>, sqlx::Error> {
        sqlx::query_as::<_, Set>(
            "SELECT * FROM program_sets WHERE id = any($1) ORDER BY array_position($2, id)",
        )
        .bind(ids)
        .bind(ids)
        .fetch_all(executor)
        .await
    }

    pub async fn delete_where_id_in(
        ids: &Vec<Uuid>,
        executor: impl Executor<'_, Database = DB>,
    ) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query("DELETE FROM program_sets WHERE id = any($1)")
            .bind(ids)
            .execute(executor)
            .await
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateSet {
    pub program_id: Uuid,
    pub movement_id: Uuid,
    #[schema(value_type = u8)]
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

fn get_day_column(day: Day) -> &'static str {
    match day {
        Day::Sunday => "set_ids_sunday",
        Day::Monday => "set_ids_monday",
        Day::Tuesday => "set_ids_tuesday",
        Day::Wednesday => "set_ids_wednesday",
        Day::Thursday => "set_ids_thursday",
        Day::Friday => "set_ids_friday",
        Day::Saturday => "set_ids_saturday",
    }
}

async fn get_set_ids(
    program_id: Uuid,
    day: Day,
    for_update: bool,
    executor: impl Executor<'_, Database = DB>,
) -> OperationResult<Option<Vec<Uuid>>> {
    let day_col = get_day_column(day);
    let lock_clause = if for_update { "FOR UPDATE" } else { "" };

    let set_ids = sqlx::query_as::<_, (Vec<Uuid>,)>(&format!(
        "SELECT {day_col} FROM programs WHERE id = $1 {lock_clause}",
    ))
    .bind(program_id)
    .fetch_optional(executor)
    .await
    .with_context(|| {
        format!(
            "failed to fetch existing set ids for day={:?} and program_id={}",
            day, program_id
        )
    })?
    .map(|id| id.0);

    Ok(set_ids)
}

async fn update_set_ids(
    program_id: Uuid,
    day: Day,
    set_ids: Vec<Uuid>,
    executor: impl Executor<'_, Database = DB>,
) -> OperationResult<PgQueryResult> {
    let day_col = get_day_column(day);
    sqlx::query(&format!("UPDATE programs SET {day_col} = $1 WHERE id = $2"))
        .bind(set_ids)
        .bind(program_id)
        .execute(executor)
        .await
        .with_context(|| {
            format!(
                "failed to update set ids for day={:?} and program_id={}",
                day, program_id
            )
        })
        .map_err(Into::into)
}

impl CreateSet {
    pub async fn insert_one(self, tx: &mut Transaction<'_, DB>) -> OperationResult<Option<Set>> {
        let set_ids = get_set_ids(self.program_id, self.day, true, &mut **tx).await?;

        if let Some(mut set_ids) = set_ids {
            let id = sqlx::query_as::<_, (Uuid,)>(
                "INSERT INTO program_sets (
                    movement_id, reps, reps_is_minimum, description, amount, percentage_of_max
                ) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id",
            )
            .bind(self.movement_id)
            .bind(self.reps)
            .bind(self.reps_is_minimum)
            .bind(&self.description)
            .bind(self.amount)
            .bind(self.percentage_of_max)
            .fetch_one(&mut **tx)
            .await
            .map_err(|e| handle_error(e, || "failed to insert new set"))?
            .0;

            set_ids.push(id);

            update_set_ids(self.program_id, self.day, set_ids, &mut **tx).await?;

            Ok(Some(Set {
                id,
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

pub async fn delete_one(
    program_id: Uuid,
    day: Day,
    id: Uuid,
    tx: &mut Transaction<'_, DB>,
) -> OperationResult<Option<()>> {
    let set_ids = get_set_ids(program_id, day, true, &mut **tx).await?;

    let res = sqlx::query("DELETE FROM program_sets WHERE id = $1")
        .bind(id)
        .execute(&mut **tx)
        .await
        .with_context(|| format!("failed to delete set with id={id}"))?;

    if res.rows_affected() == 0 {
        return Ok(None);
    }

    if let Some(mut set_ids) = set_ids {
        let idx_opt = set_ids.iter().position(|set_id| *set_id == id);

        if let Some(idx) = idx_opt {
            set_ids.remove(idx);
        }

        update_set_ids(program_id, day, set_ids, &mut **tx).await?;
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
    pub async fn update_one(
        self,
        executor: impl Executor<'_, Database = DB>,
    ) -> OperationResult<Option<Set>> {
        let res = sqlx::query(
            "UPDATE program_sets SET
            movement_id = $1,
            reps = $2,
            reps_is_minimum = $3,
            description = $4,
            amount = $5,
            percentage_of_max = $6
            WHERE id = $7
        ",
        )
        .bind(self.movement_id)
        .bind(self.reps)
        .bind(self.reps_is_minimum)
        .bind(self.description.as_ref())
        .bind(self.amount)
        .bind(self.percentage_of_max)
        .bind(self.id)
        .execute(executor)
        .await
        .map_err(|e| {
            handle_error(e, || {
                format!("failed to update set with id={id}", id = self.id)
            })
        })?;

        if res.rows_affected() == 0 {
            Ok(None)
        } else {
            Ok(Some(Set {
                id: self.id,
                movement_id: self.movement_id,
                reps: self.reps,
                reps_is_minimum: self.reps_is_minimum,
                description: self.description,
                amount: self.amount,
                percentage_of_max: self.percentage_of_max,
            }))
        }
    }
}
