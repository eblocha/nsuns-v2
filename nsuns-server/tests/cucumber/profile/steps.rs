use cucumber::{given, then, when};
use hyper::StatusCode;
use nsuns_server::{
    profiles::model::{CreateProfile, Profile},
    router::PROFILES_PATH,
};

use crate::{util::JsonBody, world::NsunsWorld};

#[when(regex = r#"^I create a profile with name "(.*)""#)]
#[given(regex = r#"^A profile with name "(.*)" exists"#)]
async fn create_profile(world: &mut NsunsWorld, name: String) {
    let create_profile = CreateProfile { name };

    let profile = world
        .client
        .post(PROFILES_PATH)
        .json_body(&create_profile)
        .send()
        .await
        .json::<_>()
        .await;

    world.profile_world.profile = Some(profile);
}

#[when(regex = r#"^I rename the profile to "(.*)""#)]
async fn update_profile(world: &mut NsunsWorld, name: String) {
    let update_profile = Profile {
        id: world.profile_world.unwrap_profile().id,
        name,
    };

    let res = world
        .client
        .put(PROFILES_PATH)
        .json_body(&update_profile)
        .send()
        .await;

    assert_eq!(StatusCode::OK, res.status());
}

#[when("I fetch all profiles")]
async fn fetch_profiles(world: &mut NsunsWorld) {
    world.profile_world.profiles = world
        .client
        .get(PROFILES_PATH)
        .send()
        .await
        .json::<_>()
        .await;
}

#[when("I delete the profile")]
async fn delete_profile(world: &mut NsunsWorld) {
    let profile_id = world.profile_world.unwrap_profile().id;

    let res = world
        .client
        .delete(&format!("{PROFILES_PATH}/{profile_id}"))
        .send()
        .await;

    assert_eq!(StatusCode::OK, res.status());
}

#[then(regex = r#"My profile has the name "(.*)""#)]
async fn get_profile(world: &mut NsunsWorld, name: String) {
    let profile_id = world.profile_world.unwrap_profile().id;

    let profile = world
        .profile_world
        .profiles
        .iter()
        .find(|profile| profile.id == profile_id)
        .expect("Profile not found");

    assert_eq!(name, profile.name);
}

#[then("My profile does not exist")]
async fn profile_not_found(world: &mut NsunsWorld) {
    let profile_id = world.profile_world.unwrap_profile().id;

    let profile = world
        .profile_world
        .profiles
        .iter()
        .find(|profile| profile.id == profile_id);

    assert!(profile.is_none());
}
