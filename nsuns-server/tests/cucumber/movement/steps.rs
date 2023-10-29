use cucumber::{given, then, when};
use hyper::StatusCode;
use nsuns_server::{
    movements::model::{CreateMovement, Movement},
    router::MOVEMENTS_PATH,
};

use crate::{util::JsonBody, world::NsunsWorld};

#[when(regex = r#"I create a movement with name "(.*)""#)]
#[given(regex = r#"A movement with name "(.*)" exists"#)]
async fn create_movement(world: &mut NsunsWorld, name: String) {
    let create_movement = CreateMovement {
        name,
        description: None,
    };

    let movement = world
        .client
        .post(MOVEMENTS_PATH)
        .json_body(&create_movement)
        .send()
        .await
        .json::<_>()
        .await;

    world.movement_world.movement = Some(movement)
}

#[given("I fetch all movements")]
#[when("I fetch all movements")]
async fn fetch_movements(world: &mut NsunsWorld) {
    world.movement_world.movements = world
        .client
        .get(MOVEMENTS_PATH)
        .send()
        .await
        .json::<_>()
        .await;
}

#[then(regex = r#"My movement has the name "(.*)""#)]
async fn have_movement(world: &mut NsunsWorld, name: String) {
    let movement_id = world.movement_world.unwrap_movement().id;

    let movement = world
        .movement_world
        .movement_by_id(movement_id)
        .expect("Movement not found");

    assert_eq!(name, movement.name);
}

#[when(regex = r#"I update the movement to have name "(.*)""#)]
async fn update_movement(world: &mut NsunsWorld, name: String) {
    let update_movement = Movement {
        id: world.movement_world.unwrap_movement().id,
        name,
        description: None,
    };

    let res = world
        .client
        .put(MOVEMENTS_PATH)
        .json_body(&update_movement)
        .send()
        .await;

    assert_eq!(StatusCode::OK, res.status());
}

#[then(regex = r#"I cannot create a movement with name "(.*)""#)]
async fn create_fail(world: &mut NsunsWorld, name: String) {
    let create_movement = CreateMovement {
        name,
        description: None,
    };

    let res = world
        .client
        .post(MOVEMENTS_PATH)
        .json_body(&create_movement)
        .send()
        .await;

    let status = res.status().as_u16();

    assert!(
        status > 399 && status < 500,
        "Status code is {status}, not 4XX"
    );
}
