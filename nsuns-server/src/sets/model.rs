use std::fmt::Display;

use anyhow::{anyhow, Context};
use axum::http::StatusCode;
use const_format::formatcp;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use sqlx::{Executor, Transaction};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::{
    auth::token::OwnerId,
    db::{
        tracing::{
            statements::{DELETE_FROM, INSERT_INTO, SELECT, UPDATE},
            InstrumentExecutor,
        },
        DB,
    },
    db_span,
    error::{ErrorWithStatus, OperationResult},
    into_log_server_error, log_server_error,
    movements::model::Movement,
    program::model::{get_set_ids, update_set_ids, Program, ProgramSetIds},
};

const TABLE: &str = "program_sets";

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

#[derive(
    Serialize_repr, Deserialize_repr, PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, sqlx::Type,
)]
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

impl Day {
    /// # Safety
    /// Caller guarantees that `value` is within the range 0..=6
    pub unsafe fn from_i16_unchecked(value: i16) -> Self {
        // explicit type annotation to use transmute more safely.
        let day: Day = unsafe { std::mem::transmute(value) };
        day
    }
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
    pub async fn select_where_id_in(
        ids: &[Uuid],
        owner_id: OwnerId,
        executor: impl Executor<'_, Database = DB>,
    ) -> Result<Vec<Set>, sqlx::Error> {
        sqlx::query_as::<_, Set>(formatcp!(
            "{SELECT} * FROM {TABLE} WHERE id = any($1) AND owner_id = $2",
        ))
        .bind(ids)
        .bind(owner_id)
        .fetch_all(executor.instrument_executor(db_span!(SELECT, TABLE)))
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
    /// Insert multiple sets into a program.
    ///
    /// The program id inside each set model is ignored in favor of the one in the argument.
    pub async fn insert_many(
        sets: &[Self],
        program_id: Uuid,
        owner_id: OwnerId,
        tx: &mut Transaction<'_, DB>,
    ) -> OperationResult<Vec<Set>> {
        // sqlx does not yet support providing iterators for bound data.
        let movement_ids: Vec<_> = sets.iter().map(|s| s.movement_id).collect();
        let reps: Vec<_> = sets.iter().map(|s| s.reps).collect();
        let reps_is_minimum: Vec<_> = sets.iter().map(|s| s.reps_is_minimum).collect();
        let description: Vec<_> = sets.iter().map(|s| s.description.as_deref()).collect();
        let amount: Vec<_> = sets.iter().map(|s| s.amount).collect();
        let percentage_of_max: Vec<_> = sets.iter().map(|s| s.percentage_of_max).collect();
        let day: Vec<_> = sets.iter().map(|s| s.day as i16).collect();

        Movement::assert_all_owner(&movement_ids, owner_id, &mut **tx).await?;

        let sets = sqlx::query_as::<_, Set>(formatcp!(
            "{INSERT_INTO} {TABLE} (
                movement_id,
                reps,
                reps_is_minimum,
                description,
                amount,
                percentage_of_max,
                day,
                program_id,
                owner_id
            ) VALUES (
                unnest($1),
                unnest($2),
                unnest($3),
                unnest($4),
                unnest($5),
                unnest($6),
                unnest($7),
                $8,
                $9
            ) RETURNING *"
        ))
        .bind(&movement_ids)
        .bind(&reps)
        .bind(&reps_is_minimum)
        .bind(&description)
        .bind(&amount)
        .bind(&percentage_of_max)
        .bind(&day)
        .bind(program_id)
        .bind(owner_id)
        .fetch_all((&mut **tx).instrument_executor(db_span!(INSERT_INTO, TABLE)))
        .await
        .map_err(|e| handle_error(e, || "failed to insert new set"))
        .map_err(log_server_error!())?;

        let program_opt = ProgramSetIds::select_one(program_id, true, owner_id, &mut **tx).await?;

        if let Some(mut program) = program_opt {
            for set in sets.iter() {
                match set.day {
                    Day::Sunday => program.set_ids_sunday.push(set.id),
                    Day::Monday => program.set_ids_monday.push(set.id),
                    Day::Tuesday => program.set_ids_tuesday.push(set.id),
                    Day::Wednesday => program.set_ids_wednesday.push(set.id),
                    Day::Thursday => program.set_ids_thursday.push(set.id),
                    Day::Friday => program.set_ids_friday.push(set.id),
                    Day::Saturday => program.set_ids_saturday.push(set.id),
                }
            }

            program.update_one(owner_id, &mut **tx).await?;

            Ok(sets)
        } else {
            Err(ErrorWithStatus::new(
                StatusCode::CONFLICT,
                anyhow!("referenced program does not exist"),
            ))
        }
    }

    pub async fn insert_one(
        self,
        owner_id: OwnerId,
        tx: &mut Transaction<'_, DB>,
    ) -> OperationResult<Option<Set>> {
        Program::assert_owner(self.program_id, owner_id, &mut **tx).await?;
        Movement::assert_owner(self.movement_id, owner_id, &mut **tx).await?;
        if let Some(percentage_of_max) = self.percentage_of_max {
            Movement::assert_owner(percentage_of_max, owner_id, &mut **tx).await?;
        }

        let set_ids = get_set_ids(self.program_id, self.day, true, owner_id, &mut **tx).await?;

        if let Some(mut set_ids) = set_ids {
            let id = sqlx::query_as::<_, (Uuid,)>(formatcp!(
                "{INSERT_INTO} {TABLE} (
                    movement_id, reps, reps_is_minimum, description, amount, percentage_of_max, program_id, day, owner_id
                ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING id",
            ))
            .bind(self.movement_id)
            .bind(self.reps)
            .bind(self.reps_is_minimum)
            .bind(&self.description)
            .bind(self.amount)
            .bind(self.percentage_of_max)
            .bind(self.program_id)
            .bind(self.day)
            .bind(owner_id)
            .fetch_one((&mut **tx).instrument_executor(db_span!(INSERT_INTO, TABLE)))
            .await
            .map_err(|e| handle_error(e, || "failed to insert new set"))
            .map_err(log_server_error!())?
            .0;

            set_ids.push(id);

            update_set_ids(self.program_id, self.day, &set_ids, owner_id, &mut **tx).await?;

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

pub async fn delete_one(
    id: Uuid,
    owner_id: OwnerId,
    tx: &mut Transaction<'_, DB>,
) -> OperationResult<Option<()>> {
    let res = sqlx::query_as::<_, (Uuid, Day)>(formatcp!(
        "{DELETE_FROM} {TABLE} WHERE id = $1 AND owner_id = $2 RETURNING program_id, day",
    ))
    .bind(id)
    .bind(owner_id)
    .fetch_optional((&mut **tx).instrument_executor(db_span!(DELETE_FROM, TABLE)))
    .await
    .with_context(|| format!("failed to delete set with id={id}"))
    .map_err(into_log_server_error!())?;

    if let Some((program_id, day)) = res {
        let set_ids = get_set_ids(program_id, day, true, owner_id, &mut **tx).await?;

        if let Some(set_ids) = set_ids {
            let set_ids: Vec<_> = set_ids.into_iter().filter(|set_id| *set_id != id).collect();
            update_set_ids(program_id, day, &set_ids, owner_id, &mut **tx).await?;
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
    pub async fn update_one(
        self,
        owner_id: OwnerId,
        tx: &mut Transaction<'_, DB>,
    ) -> OperationResult<Option<Set>> {
        Movement::assert_owner(self.movement_id, owner_id, &mut **tx).await?;
        if let Some(percentage_of_max) = self.percentage_of_max {
            Movement::assert_owner(percentage_of_max, owner_id, &mut **tx).await?;
        }

        let res = sqlx::query_as::<_, (Uuid, Day)>(formatcp!(
            "{UPDATE} {TABLE} SET
            movement_id = $1,
            reps = $2,
            reps_is_minimum = $3,
            description = $4,
            amount = $5,
            percentage_of_max = $6
            WHERE id = $7 AND owner_id = $8
            RETURNING program_id, day
        ",
        ))
        .bind(self.movement_id)
        .bind(self.reps)
        .bind(self.reps_is_minimum)
        .bind(self.description.as_ref())
        .bind(self.amount)
        .bind(self.percentage_of_max)
        .bind(self.id)
        .bind(owner_id)
        .fetch_optional((&mut **tx).instrument_executor(db_span!(UPDATE, TABLE)))
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
