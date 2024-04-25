use cucumber::{given, then, when};
use hyper::StatusCode;
use nsuns_server::{
    program::model::{CreateProgram, UpdateProgram},
    router::PROGRAMS_PATH,
};

use crate::{util::{Auth, JsonBody}, world::NsunsWorld};

#[when(regex = r#"^I create a program with name "(.*)""#)]
#[given(regex = r#"A program with name "(.*)" exists"#)]
async fn create_program(world: &mut NsunsWorld, name: String) {
    let create_program = CreateProgram {
        owner: world.profile_world.unwrap_profile().id,
        name,
        description: None,
    };

    let program_meta = world
        .client
        .post(PROGRAMS_PATH)
        .json_body(&create_program)
        .authed(world)
        .send()
        .await
        .json::<_>()
        .await;

    world.program_world.program_meta = Some(program_meta);
}

#[when("I fetch my programs")]
async fn fetch_programs(world: &mut NsunsWorld) {
    let profile_id = world.profile_world.unwrap_profile().id;

    world.program_world.programs_for_profile = world
        .client
        .get(&format!("{PROGRAMS_PATH}?profileId={profile_id}"))
        .authed(world)
        .send()
        .await
        .json::<_>()
        .await;
}

#[when("I fetch my program summary")]
async fn fetch_program_summary(world: &mut NsunsWorld) {
    let program_id = world.program_world.unwrap_program_meta().id;

    world.program_world.program_summary = Some(
        world
            .client
            .get(&format!("{PROGRAMS_PATH}/{program_id}"))
            .authed(world)
            .send()
            .await
            .json::<_>()
            .await,
    );
}

#[when("I delete my program")]
async fn delete_program(world: &mut NsunsWorld) {
    let program_id = world.program_world.unwrap_program_meta().id;

    let res = world
        .client
        .delete(&format!("{PROGRAMS_PATH}/{program_id}"))
        .authed(world)
        .send()
        .await;
    assert_eq!(StatusCode::OK, res.status());
}

#[when(regex = r#"I update my program to have name "(.*)""#)]
async fn update_program(world: &mut NsunsWorld, name: String) {
    let res = world
        .client
        .put(PROGRAMS_PATH)
        .json_body(&UpdateProgram {
            description: None,
            id: world.program_world.unwrap_program_meta().id,
            name,
        })
        .authed(world)
        .send()
        .await;

    assert_eq!(StatusCode::OK, res.status());
}

#[then(regex = r#"My program has the name "(.*)""#)]
async fn get_program(world: &mut NsunsWorld, name: String) {
    let program_id = world.program_world.unwrap_program_meta().id;

    let program_meta = world
        .program_world
        .programs_for_profile
        .iter()
        .find(|program_meta| program_meta.id == program_id)
        .expect("Program not found");

    assert_eq!(name, program_meta.name);
}

#[then("My program does not exist")]
async fn program_not_found(world: &mut NsunsWorld) {
    let program_id = world.program_world.unwrap_program_meta().id;

    let program_meta = world
        .program_world
        .programs_for_profile
        .iter()
        .find(|program_meta| program_meta.id == program_id);

    assert!(program_meta.is_none(), "Program exists");
}
