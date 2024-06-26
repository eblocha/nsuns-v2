use std::{collections::HashMap, fmt::Display};

use anyhow::{anyhow, Context};
use axum::http::StatusCode;
use chrono::naive::serde::ts_milliseconds;
use chrono::NaiveDateTime;
use const_format::formatcp;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgQueryResult, Executor, Transaction};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::{
    assert_all_owner, assert_owner,
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
    profiles::model::Profile,
    sets::model::{Day, Set},
    vec::MoveWithin,
};

const TABLE: &str = "programs";

fn handle_error<F, C>(e: sqlx::Error, context: F) -> ErrorWithStatus<anyhow::Error>
where
    F: FnOnce() -> C,
    C: Display + Send + Sync + 'static,
{
    match e {
        sqlx::Error::Database(e) if e.is_foreign_key_violation() => ErrorWithStatus::new(
            StatusCode::BAD_REQUEST,
            anyhow!("profileId provided does not exist"),
        ),
        _ => anyhow!(e).context(context()).into(),
    }
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Program {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub owner: Uuid,
    pub created_on: NaiveDateTime,
    pub set_ids_sunday: Vec<Uuid>,
    pub set_ids_monday: Vec<Uuid>,
    pub set_ids_tuesday: Vec<Uuid>,
    pub set_ids_wednesday: Vec<Uuid>,
    pub set_ids_thursday: Vec<Uuid>,
    pub set_ids_friday: Vec<Uuid>,
    pub set_ids_saturday: Vec<Uuid>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ProgramSetIds {
    pub id: Uuid,
    pub set_ids_sunday: Vec<Uuid>,
    pub set_ids_monday: Vec<Uuid>,
    pub set_ids_tuesday: Vec<Uuid>,
    pub set_ids_wednesday: Vec<Uuid>,
    pub set_ids_thursday: Vec<Uuid>,
    pub set_ids_friday: Vec<Uuid>,
    pub set_ids_saturday: Vec<Uuid>,
}

impl ProgramSetIds {
    pub async fn select_one(
        id: Uuid,
        for_update: bool,
        owner_id: OwnerId,
        executor: impl Executor<'_, Database = DB>,
    ) -> OperationResult<Option<Self>> {
        let lock_clause = if for_update { "FOR UPDATE" } else { "" };

        sqlx::query_as::<_, Self>(&format!(
            "{SELECT}
            id,
            set_ids_sunday,
            set_ids_monday,
            set_ids_tuesday,
            set_ids_wednesday,
            set_ids_thursday,
            set_ids_friday,
            set_ids_saturday
            FROM {TABLE}
            WHERE id = $1 AND owner_id = $2 {lock_clause}"
        ))
        .bind(id)
        .bind(owner_id)
        .fetch_optional(executor.instrument_executor(db_span!(SELECT, TABLE)))
        .await
        .with_context(|| format!("failed to fetch program with id={id}"))
        .map_err(into_log_server_error!())
    }

    /// Blindly update set ids on a program.
    ///
    /// This does _NOT_ check that the sets pointed to are valid, nor remove any dropped sets.
    ///
    /// This is primarily intended for bootstrapping a new program from a template.
    pub async fn update_one(
        &self,
        owner_id: OwnerId,
        executor: impl Executor<'_, Database = DB>,
    ) -> OperationResult<()> {
        sqlx::query(formatcp!(
            "{UPDATE} {TABLE} SET
            set_ids_sunday = $1,
            set_ids_monday = $2,
            set_ids_tuesday = $3,
            set_ids_wednesday = $4,
            set_ids_thursday = $5,
            set_ids_friday = $6,
            set_ids_saturday = $7
            WHERE id = $8 AND owner_id = $9"
        ))
        .bind(&self.set_ids_sunday)
        .bind(&self.set_ids_monday)
        .bind(&self.set_ids_tuesday)
        .bind(&self.set_ids_wednesday)
        .bind(&self.set_ids_thursday)
        .bind(&self.set_ids_friday)
        .bind(&self.set_ids_saturday)
        .bind(self.id)
        .bind(owner_id)
        .execute(executor.instrument_executor(db_span!(UPDATE, TABLE)))
        .await
        .with_context(|| format!("failed to fetch program with id={}", self.id))
        .map_err(into_log_server_error!())
        .map(|_| ())
    }
}

impl Program {
    pub async fn select_one(
        id: Uuid,
        owner_id: OwnerId,
        executor: impl Executor<'_, Database = DB>,
    ) -> OperationResult<Option<Self>> {
        sqlx::query_as::<_, Self>(formatcp!(
            "{SELECT} * from {TABLE} WHERE id = $1 AND owner_id = $2"
        ))
        .bind(id)
        .bind(owner_id)
        .fetch_optional(executor.instrument_executor(db_span!(SELECT, TABLE)))
        .await
        .with_context(|| format!("failed to fetch program with id={id}"))
        .map_err(into_log_server_error!())
    }

    pub async fn assert_owner(
        id: Uuid,
        owner_id: OwnerId,
        executor: impl Executor<'_, Database = DB>,
    ) -> OperationResult<()> {
        assert_owner!(TABLE, "program", id, owner_id, executor)
    }

    pub async fn assert_all_owner(
        ids: &[Uuid],
        owner_id: OwnerId,
        executor: impl Executor<'_, Database = DB>,
    ) -> OperationResult<()> {
        assert_all_owner!(TABLE, "program", ids, owner_id, executor)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow, ToSchema, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ProgramMeta {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub owner: Uuid,
    #[schema(value_type = i64)]
    #[serde(with = "ts_milliseconds")]
    pub created_on: NaiveDateTime,
}

const PROGRAM_META_COLS: &str = "id, name, description, owner, created_on";

impl ProgramMeta {
    pub async fn select_all_for_profile(
        profile_id: Uuid,
        owner_id: OwnerId,
        executor: impl Executor<'_, Database = DB>,
    ) -> OperationResult<Vec<Self>> {
        sqlx::query_as::<_, Self>(formatcp!(
            "{SELECT} {PROGRAM_META_COLS} FROM {TABLE} WHERE owner = $1 AND owner_id = $2 ORDER BY created_on",
        ))
        .bind(profile_id)
        .bind(owner_id)
        .fetch_all(executor.instrument_executor(db_span!(SELECT, TABLE)))
        .await
        .with_context(|| format!("failed to select program with owner id={profile_id}"))
        .map_err(into_log_server_error!())
    }

    pub async fn select_one(
        id: Uuid,
        owner_id: OwnerId,
        executor: impl Executor<'_, Database = DB>,
    ) -> OperationResult<Option<Self>> {
        sqlx::query_as::<_, Self>(formatcp!(
            "{SELECT} {PROGRAM_META_COLS} FROM {TABLE} WHERE id = $1 AND owner_id = $2",
        ))
        .bind(id)
        .bind(owner_id)
        .fetch_optional(executor.instrument_executor(db_span!(SELECT, TABLE)))
        .await
        .with_context(|| format!("failed to fetch program with id={id}"))
        .map_err(into_log_server_error!())
    }
}

impl From<Program> for ProgramMeta {
    fn from(value: Program) -> Self {
        ProgramMeta {
            id: value.id,
            name: value.name,
            description: value.description,
            owner: value.owner,
            created_on: value.created_on,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateProgram {
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(length(min = 1))]
    pub description: Option<String>,
    pub owner: Uuid,
}

impl CreateProgram {
    pub async fn insert_one(
        self,
        owner_id: OwnerId,
        tx: &mut Transaction<'_, DB>,
    ) -> OperationResult<ProgramMeta> {
        Profile::assert_owner(self.owner, owner_id, &mut **tx).await?;
        sqlx::query_as::<_, ProgramMeta>(formatcp!(
            "{INSERT_INTO} {TABLE} (name, description, owner, owner_id) VALUES ($1, $2, $3, $4) RETURNING {PROGRAM_META_COLS}",
        ))
        .bind(self.name)
        .bind(self.description)
        .bind(self.owner)
        .bind(owner_id)
        .fetch_one((&mut **tx).instrument_executor(db_span!(INSERT_INTO, TABLE)))
        .await
        .map_err(|e| handle_error(e, || "failed to create program"))
        .map_err(log_server_error!())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProgram {
    pub id: Uuid,
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(length(min = 1))]
    pub description: Option<String>,
}

impl UpdateProgram {
    pub async fn update_one(
        self,
        owner_id: OwnerId,
        executor: impl Executor<'_, Database = DB>,
    ) -> OperationResult<Option<ProgramMeta>> {
        sqlx::query_as::<_, ProgramMeta>(formatcp!(
            "{UPDATE} {TABLE} SET name = $1, description = $2 WHERE id = $3 AND owner_id = $4 RETURNING {PROGRAM_META_COLS}",
        ))
        .bind(self.name)
        .bind(self.description)
        .bind(self.id)
        .bind(owner_id)
        .fetch_optional(executor.instrument_executor(db_span!(UPDATE, TABLE)))
        .await
        .map_err(|e| {
            handle_error(e, || {
                format!("failed to update program with id={id}", id = self.id)
            })
        })
        .map_err(log_server_error!())
    }
}

pub async fn delete_one(
    id: Uuid,
    owner_id: OwnerId,
    executor: impl Executor<'_, Database = DB>,
) -> OperationResult<Option<ProgramMeta>> {
    sqlx::query_as::<_, ProgramMeta>(formatcp!(
        "{DELETE_FROM} {TABLE} WHERE id = $1 AND owner_id = $2 RETURNING {PROGRAM_META_COLS}"
    ))
    .bind(id)
    .bind(owner_id)
    .fetch_optional(executor.instrument_executor(db_span!(DELETE_FROM, TABLE)))
    .await
    .with_context(|| format!("failed to delete program with id={id}"))
    .map_err(into_log_server_error!())
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ProgramSummary {
    pub program: ProgramMeta,
    pub sets_sunday: Vec<Set>,
    pub sets_monday: Vec<Set>,
    pub sets_tuesday: Vec<Set>,
    pub sets_wednesday: Vec<Set>,
    pub sets_thursday: Vec<Set>,
    pub sets_friday: Vec<Set>,
    pub sets_saturday: Vec<Set>,
}

pub async fn gather_program_summary(
    id: Uuid,
    owner_id: OwnerId,
    tx: &mut Transaction<'_, DB>,
) -> OperationResult<Option<ProgramSummary>> {
    let program_opt = Program::select_one(id, owner_id, &mut **tx).await?;

    let get_ctx = || format!("failed to fetch sets for program with id={id}");

    if let Some(program) = program_opt {
        let all_ids: Vec<_> = [
            program.set_ids_sunday.clone(),
            program.set_ids_monday.clone(),
            program.set_ids_tuesday.clone(),
            program.set_ids_wednesday.clone(),
            program.set_ids_thursday.clone(),
            program.set_ids_friday.clone(),
            program.set_ids_saturday.clone(),
        ]
        .concat();

        let all_sets = Set::select_where_id_in(&all_ids, owner_id, &mut **tx)
            .await
            .with_context(get_ctx)
            .map_err(into_log_server_error!())?;

        let mut set_map: HashMap<Uuid, Set> = HashMap::with_capacity(all_sets.len());

        for set in all_sets {
            set_map.insert(set.id, set);
        }

        let sets_sunday = program
            .set_ids_sunday
            .iter()
            .filter_map(|set_id| set_map.remove(set_id))
            .collect();

        let sets_monday = program
            .set_ids_monday
            .iter()
            .filter_map(|set_id| set_map.remove(set_id))
            .collect();

        let sets_tuesday = program
            .set_ids_tuesday
            .iter()
            .filter_map(|set_id| set_map.remove(set_id))
            .collect();

        let sets_wednesday = program
            .set_ids_wednesday
            .iter()
            .filter_map(|set_id| set_map.remove(set_id))
            .collect();

        let sets_thursday = program
            .set_ids_thursday
            .iter()
            .filter_map(|set_id| set_map.remove(set_id))
            .collect();

        let sets_friday = program
            .set_ids_friday
            .iter()
            .filter_map(|set_id| set_map.remove(set_id))
            .collect();

        let sets_saturday = program
            .set_ids_saturday
            .iter()
            .filter_map(|set_id| set_map.remove(set_id))
            .collect();

        Ok(Some(ProgramSummary {
            program: program.into(),
            sets_sunday,
            sets_monday,
            sets_tuesday,
            sets_wednesday,
            sets_thursday,
            sets_friday,
            sets_saturday,
        }))
    } else {
        Ok(None)
    }
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

pub async fn get_set_ids(
    program_id: Uuid,
    day: Day,
    for_update: bool,
    owner_id: OwnerId,
    executor: impl Executor<'_, Database = DB>,
) -> OperationResult<Option<Vec<Uuid>>> {
    let day_col = get_day_column(day);
    let lock_clause = if for_update { "FOR UPDATE" } else { "" };

    let set_ids = sqlx::query_as::<_, (Vec<Uuid>,)>(&format!(
        "{SELECT} {day_col} FROM {TABLE} WHERE id = $1 AND owner_id = $2 {lock_clause}",
    ))
    .bind(program_id)
    .bind(owner_id)
    .fetch_optional(executor.instrument_executor(db_span!(SELECT, TABLE)))
    .await
    .with_context(|| {
        format!("failed to fetch existing set ids for day={day:?} and program_id={program_id}",)
    })
    .map_err(into_log_server_error!())?
    .map(|id| id.0);

    Ok(set_ids)
}

pub async fn update_set_ids(
    program_id: Uuid,
    day: Day,
    set_ids: &[Uuid],
    owner_id: OwnerId,
    executor: impl Executor<'_, Database = DB>,
) -> OperationResult<PgQueryResult> {
    let day_col = get_day_column(day);
    sqlx::query(&format!(
        "{UPDATE} {TABLE} SET {day_col} = $1 WHERE id = $2 AND owner_id = $3"
    ))
    .bind(set_ids)
    .bind(program_id)
    .bind(owner_id)
    .execute(executor.instrument_executor(db_span!(UPDATE, TABLE)))
    .await
    .with_context(|| {
        format!("failed to update set ids for day={day:?} and program_id={program_id}",)
    })
    .map_err(into_log_server_error!())
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ReorderSets {
    pub program_id: Uuid,
    #[schema(value_type = i16)]
    pub day: Day,
    #[validate(range(min = 0))]
    pub from: usize,
    #[validate(range(min = 0))]
    pub to: usize,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SetId(Uuid);

impl ReorderSets {
    pub async fn reorder<'a>(
        &self,
        owner_id: OwnerId,
        tx: &mut Transaction<'a, DB>,
    ) -> OperationResult<Option<Vec<SetId>>> {
        self.reorder_unlogged(owner_id, tx)
            .await
            .map_err(log_server_error!())
    }

    async fn reorder_unlogged<'a>(
        &self,
        owner_id: OwnerId,
        tx: &mut Transaction<'a, DB>,
    ) -> OperationResult<Option<Vec<SetId>>> {
        if let Some(mut set_ids) =
            get_set_ids(self.program_id, self.day, true, owner_id, &mut **tx).await?
        {
            if self.from >= set_ids.len() || self.to >= set_ids.len() {
                return Err(ErrorWithStatus::new(
                    StatusCode::CONFLICT,
                    anyhow!("index out of bounds"),
                ));
            }

            if set_ids.move_within(self.from, self.to) {
                update_set_ids(self.program_id, self.day, &set_ids, owner_id, &mut **tx).await?;
            }

            return Ok(Some(set_ids.into_iter().map(SetId).collect()));
        }

        Ok(None)
    }
}
