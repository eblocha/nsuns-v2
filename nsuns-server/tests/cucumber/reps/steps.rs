use cucumber::given;
use hyper::StatusCode;
use nsuns_server::{reps::model::CreateReps, router::REPS_PATH};

use crate::{util::{Auth, JsonBody}, world::NsunsWorld};

#[given(regex = r#"I have (\d+) reps? in "(.*)""#)]
async fn create_reps(world: &mut NsunsWorld, reps: i32, movement_name: String) {
    let profile_id = world.profile_world.unwrap_profile().id;
    let movement_id = world
        .movement_world
        .movement_by_name(&movement_name)
        .expect("Movement does not exist")
        .id;

    let res = world
        .client
        .post(REPS_PATH)
        .json_body(&CreateReps {
            amount: Some(reps),
            movement_id,
            profile_id,
        })
        .authed(world)
        .send()
        .await;

    assert_eq!(StatusCode::CREATED, res.status());
}
