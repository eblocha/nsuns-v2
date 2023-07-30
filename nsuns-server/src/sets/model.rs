use anyhow::Context;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use sqlx::{Executor, Transaction};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::{
    db::DB,
    error::{IntoResult, Result},
};

#[derive(Debug, Serialize_repr, Deserialize_repr, Clone, Copy, sqlx::Type)]
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

#[derive(Debug, Serialize, Clone, sqlx::FromRow, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Set {
    pub id: Uuid,
    pub program_id: Uuid,
    pub movement_id: Uuid,
    #[schema(value_type = i16)]
    pub day: Day,
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
    ) -> Result<Vec<Set>> {
        sqlx::query_as::<_, Self>(
            "SELECT * FROM program_sets WHERE program_id = $1 ORDER BY day, ordering",
        )
        .bind(program_id)
        .fetch_all(executor)
        .await
        .with_context(|| format!("failed to select sets with program_id={}", program_id))
        .into_result()
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
    pub async fn insert_one(self, tx: &mut Transaction<'_, DB>) -> Result<Set> {
        // get the max `ordering` value for the program and day
        let ordering = sqlx::query_as::<_, (Option<i32>,)>(
            "SELECT MAX(ordering) FROM program_sets WHERE program_id = $1 AND day = $2",
        )
        .bind(self.program_id)
        .bind(self.day)
        .fetch_one(&mut **tx)
        .await
        .with_context(|| {
            format!(
                "failed to fetch max ordered set with program_id={} and day={:?}",
                self.program_id, self.day
            )
        })?
        .0
        .map(|value| value + 1)
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
        .with_context(|| "failed to insert new set")?
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

pub async fn delete_one(id: Uuid, tx: &mut Transaction<'_, DB>) -> Result<Option<Set>> {
    let set_opt = sqlx::query_as::<_, Set>("DELETE FROM program_sets WHERE id = $1 RETURNING *")
        .bind(id)
        .fetch_optional(&mut **tx)
        .await
        .with_context(|| format!("failed to delete set with id={}", id))?;

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

impl UpdateSet {
    pub async fn update_one(
        self,
        executor: impl Executor<'_, Database = DB>,
    ) -> Result<Option<Set>> {
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
        .with_context(|| format!("failed to update set with id={}", self.id))?;

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
