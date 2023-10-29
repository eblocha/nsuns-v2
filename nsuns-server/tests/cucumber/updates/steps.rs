use cucumber::when;
use nsuns_server::{router::UPDATES_PATH, updates::handler::Updates};

use crate::{util::JsonBody, world::NsunsWorld};

fn get_updates(world: &NsunsWorld) -> Updates {
    let profile_id = world.profile_world.unwrap_profile().id;
    let movement_ids = world
        .movement_world
        .movements
        .iter()
        .map(|movement| movement.id)
        .collect();

    Updates {
        movement_ids,
        profile_id,
    }
}

#[when("I run updates")]
async fn run_updates(world: &mut NsunsWorld) {
    world
        .client
        .post(UPDATES_PATH)
        .json_body(&get_updates(world))
        .send()
        .await;
}

#[when("I undo updates")]
async fn undo_updates(world: &mut NsunsWorld) {
    world
        .client
        .delete(UPDATES_PATH)
        .json_body(&get_updates(world))
        .send()
        .await;
}
