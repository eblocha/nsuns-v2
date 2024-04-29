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
    sets::model::{CreateSet, Day}, validation::Validated,
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
    name: String,
    owner: Uuid,
    days: [DayTemplate; 7],
    movements: Vec<MovementTemplate>,
}

fn validate_set_template(set: &SetTemplate, movements_len: usize) -> Result<(), ValidationErrors> {
    let mut set_errs = set.validate().err().unwrap_or_default();

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

impl Validated<TemplatedProgram> {
    pub async fn insert(
        self,
        owner_id: OwnerId,
        tx: &mut Transaction<'_, DB>,
    ) -> OperationResult<ProgramMeta> {
        let template = self.into_inner();

        // We don't need to assert any ownership, since we are deferring resource creation to respective models.
        let mut movement_ids: Vec<_> = (0..template.movements.len()).map(|_| Uuid::nil()).collect();

        let mut new_movement_indexes: Vec<usize> = Vec::new();
        let mut movements_to_create: Vec<CreateMovement> = Vec::new();

        // verify all referenced movements are owned by this owner, or add it to the movements to create
        for (index, movement) in template.movements.into_iter().enumerate() {
            match movement {
                MovementTemplate::Ref(MovementRef { id }) => {
                    movement_ids[index] = id;
                }
                MovementTemplate::New(movement) => {
                    movements_to_create.push(movement);
                    new_movement_indexes.push(index);
                }
            }
        }

        let new_movements =
            CreateMovement::insert_many(&movements_to_create, owner_id, &mut **tx).await?;

        debug_assert_eq!(
            new_movement_indexes.len(),
            new_movements.len(),
            "Did not create the expected number of movements!"
        );

        for (idx, movement) in new_movements.iter().enumerate() {
            // new_movement length will be equal to `new_movement_indexes` length - query would fail otherwise.
            let index = new_movement_indexes[idx];
            movement_ids[index] = movement.id;
        }

        // create the program - this checks ownership of the profile
        let program_meta = CreateProgram {
            description: None,
            name: template.name,
            owner: template.owner,
        }
        .insert_one(owner_id, tx)
        .await?;

        let sets_to_create: Vec<CreateSet> = template
            .days
            .into_iter()
            .enumerate()
            .flat_map(|(index, day_template)| {
                // SAFETY: `self.days` is an array of length 7, so `index` can never be more than 6.
                let day: Day = unsafe { Day::from_i16_unchecked(index as i16) };
                let movement_ids = &movement_ids;

                day_template
                    .sets
                    .into_iter()
                    .map(move |set| {
                        CreateSet {
                            amount: set.amount,
                            day,
                            description: set.description,
                            movement_id: movement_ids[set.movement_index], // validated by `Validate` impl
                            percentage_of_max: set
                                .percentage_of_max_index
                                .map(|idx| movement_ids[idx]), // validated by `Validate` impl
                            program_id: program_meta.id,
                            reps: set.reps,
                            reps_is_minimum: set.reps_is_minimum,
                        }
                    })
            })
            .collect();

        CreateSet::insert_many(&sets_to_create, program_meta.id, owner_id, tx).await?;

        Ok(program_meta)
    }
}
