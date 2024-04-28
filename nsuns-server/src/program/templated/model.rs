use serde::{Deserialize, Serialize};
use sqlx::Transaction;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::{Validate, ValidationError, ValidationErrors};

use crate::{
    auth::token::OwnerId,
    db::DB,
    error::OperationResult,
    movements::model::CreateMovement,
    program::model::{CreateProgram, ProgramMeta},
    sets::model::{CreateSet, Day},
};

#[derive(Debug, Deserialize, Serialize, Clone, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SetTemplate {
    pub movement_index: usize,
    pub percentage_of_max_index: Option<usize>,
    #[validate(range(min = 0))]
    pub reps: Option<i32>,
    pub reps_is_minimum: bool,
    #[validate(length(min = 1))]
    pub description: Option<String>,
    #[validate(range(min = 0))]
    pub amount: f64,
}

#[derive(Debug, Deserialize, Serialize, Clone, ToSchema)]
pub struct DayTemplate {
    pub sets: Vec<SetTemplate>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, ToSchema)]
pub struct MovementRef {
    pub id: Uuid,
}

#[derive(Debug, Deserialize, Serialize, Clone, ToSchema)]
#[serde(tag = "type")]
pub enum MovementTemplate {
    #[serde(rename = "existing")]
    Ref(MovementRef),
    #[serde(rename = "new")]
    New(CreateMovement),
}

#[derive(Debug, Deserialize, Serialize, Clone, ToSchema)]
pub struct TemplatedProgram {
    pub name: String,
    pub owner: Uuid,
    pub days: [DayTemplate; 7],
    pub movements: Vec<MovementTemplate>,
}

fn validate_set_template(set: &SetTemplate, movements_len: usize) -> Result<(), ValidationErrors> {
    let mut set_errs = set.validate().err().unwrap_or_else(ValidationErrors::new);

    if set.movement_index >= movements_len {
        set_errs.add(
            "movement_index",
            ValidationError::new("movement_index out of bounds"),
        )
    }

    if set
        .percentage_of_max_index
        .map(|idx| idx >= movements_len)
        .unwrap_or(false)
    {
        set_errs.add(
            "percentage_of_max_index",
            ValidationError::new("percentage_of_max_index out of bounds"),
        )
    }

    if set_errs.is_empty() {
        Ok(())
    } else {
        Err(set_errs)
    }
}

impl Validate for TemplatedProgram {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        let movements_len = self.movements.len();

        let days_errs: Vec<_> = self
            .days
            .iter()
            .map(|day| {
                let sets_errs: Vec<_> = day
                    .sets
                    .iter()
                    .map(|set| validate_set_template(set, movements_len))
                    .collect();

                ValidationErrors::merge_all(Ok(()), "sets", sets_errs)
            })
            .collect();

        let movements_errs: Vec<_> = self
            .movements
            .iter()
            .map(|movement| match movement {
                MovementTemplate::Ref(_) => Ok(()),
                MovementTemplate::New(create_movement) => create_movement.validate(),
            })
            .collect();

        let base = ValidationErrors::merge_all(Ok(()), "movements", movements_errs);

        ValidationErrors::merge_all(base, "days", days_errs)
    }
}

impl TemplatedProgram {
    pub async fn insert(
        self,
        owner_id: OwnerId,
        tx: &mut Transaction<'_, DB>,
    ) -> OperationResult<ProgramMeta> {
        // We don't need to assert any ownership, since we are deferring resource creation to respective models.
        let mut movement_ids = Vec::with_capacity(self.movements.len());

        // verify all referenced movements are owned by this owner, or create the movement
        for movement in self.movements.into_iter() {
            match movement {
                MovementTemplate::Ref(MovementRef { id }) => {
                    movement_ids.push(id);
                }
                MovementTemplate::New(movement) => {
                    // TODO optimize this to create all at once
                    let movement = movement.insert_one(owner_id, &mut **tx).await?;
                    movement_ids.push(movement.id);
                }
            }
        }

        // create the program
        let program_meta = CreateProgram {
            description: None,
            name: self.name,
            owner: self.owner,
        }
        .insert_one(owner_id, tx)
        .await?;

        // add sets
        for (index, day_template) in self.days.into_iter().enumerate() {
            // SAFETY: `self.days` is an array of length 7, so `index` can never be more than 6.
            let day: Day = unsafe { Day::from_i16_unchecked(index as i16) };
            for set in day_template.sets.into_iter() {
                // TODO optimize this to create all at once
                CreateSet {
                    amount: set.amount,
                    day,
                    description: set.description,
                    movement_id: movement_ids[set.movement_index], // validated by `Validate` impl
                    percentage_of_max: set.percentage_of_max_index.map(|idx| movement_ids[idx]), // validated by `Validate` impl
                    program_id: program_meta.id,
                    reps: set.reps,
                    reps_is_minimum: set.reps_is_minimum,
                }
                .insert_one(owner_id, tx)
                .await?;
            }
        }

        Ok(program_meta)
    }
}
