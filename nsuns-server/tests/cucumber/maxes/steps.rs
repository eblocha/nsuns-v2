use cucumber::{given, then, when};
use hyper::StatusCode;
use nsuns_server::{maxes::model::CreateMax, router::MAXES_PATH};

use crate::{
    util::{Auth, JsonBody},
    world::NsunsWorld,
};

#[given(regex = r#"I have a max of (\d+) in "(.*)""#)]
async fn create_maxes(world: &mut NsunsWorld, amount: f64, movement_name: String) {
    let profile_id = world.profile_world.unwrap_profile().id;
    let movement_id = world
        .movement_world
        .movement_by_name(&movement_name)
        .expect("Movement does not exist")
        .id;

    let res = world
        .client
        .post(MAXES_PATH)
        .json_body(&CreateMax {
            profile_id,
            movement_id,
            amount,
        })
        .authed(world)
        .send()
        .await;

    assert_eq!(StatusCode::CREATED, res.status());
}

#[when("I fetch my maxes")]
async fn fetch_maxes(world: &mut NsunsWorld) {
    let profile_id = world.profile_world.unwrap_profile().id;

    world.maxes_world.maxes = world
        .client
        .get(&format!("{MAXES_PATH}?profileId={profile_id}"))
        .authed(world)
        .send()
        .await
        .json()
        .await;
}

#[then(regex = r#"My "(.*)" max is (\d+)"#)]
async fn latest_max_is(world: &mut NsunsWorld, movement_name: String, amount: f64) {
    let movement_id = world
        .movement_world
        .movement_by_name(&movement_name)
        .expect("Movement not found")
        .id;

    let latest = world
        .maxes_world
        .maxes
        .iter()
        .filter(|max| max.movement_id == movement_id)
        .last()
        .expect("No maxes found");

    assert_eq!(amount, latest.amount);
}
