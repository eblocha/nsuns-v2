use anyhow::Context;
use serde::{Deserialize, Serialize};
use sqlx::{Executor, Transaction};

use crate::{
    db::DB,
    error::{IntoResult, Result},
};

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Set {
    pub id: i32,
    pub program_id: i32,
    pub movement_id: i32,
    pub day: i32,
    pub ordering: i32,
    pub reps: Option<i32>,
    pub reps_is_minimum: bool,
    pub description: Option<String>,
    pub amount: f64,
    pub percentage_of_max: Option<i32>,
}

impl Set {
    pub async fn select_for_program(
        program_id: i32,
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

#[derive(Debug, Deserialize, Serialize, Clone, validator::Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateSet {
    pub program_id: i32,
    pub movement_id: i32,
    #[validate(range(min = 0, max = 6))]
    pub day: i32,
    #[validate(range(min = 0))]
    pub reps: Option<i32>,
    #[serde(default)]
    pub reps_is_minimum: bool,
    pub description: Option<String>,
    pub amount: f64,
    pub percentage_of_max: Option<i32>,
}

impl CreateSet {
    pub async fn insert_one(self, tx: &mut Transaction<'_, DB>) -> Result<Set> {
        // get the max `ordering` value for the program and day
        let ordering = sqlx::query_as::<_, (i32,)>(
            "SELECT MAX(ordering) FROM program_sets WHERE program_id = $1 AND day = $2",
        )
        .bind(self.program_id)
        .bind(self.day)
        .fetch_optional(&mut **tx)
        .await
        .with_context(|| {
            format!(
                "failed to fetch max ordered set with program_id={} and day={}",
                self.program_id, self.day
            )
        })?
        .map(|(order,)| order + 1)
        .unwrap_or(0);

        let id = sqlx::query_as::<_, (i32,)>(
            "INSERT INTO program_sets SET (
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
            percentage_of_max: self.percentage_of_max
        })
    }
}

pub async fn delete_one(id: i32, tx: &mut Transaction<'_, DB>) -> Result<Option<()>> {
    let opt = sqlx::query_as::<_, (i32, i32, i32)>(
        "DELETE FROM program_sets WHERE id = $1 RETURNING ordering, program_id, day",
    )
    .bind(id)
    .fetch_optional(&mut **tx)
    .await
    .with_context(|| format!("failed to delete set with id={}", id))?;

    if let Some((ordering, program_id, day)) = opt {
        // decrement any sets with ordering > this one
        sqlx::query("UPDATE program_sets SET ordering = ordering - 1 WHERE ordering > $1 AND program_id = $2 AND day = $3")
            .bind(ordering)
            .bind(program_id)
            .bind(day)
            .execute(&mut **tx)
            .await
            .with_context(|| "failed to decrement remaining set ordering")?;
        Ok(Some(()))
    } else {
        Ok(None)
    }
}
