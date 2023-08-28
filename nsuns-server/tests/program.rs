mod setup;

use axum_test_helper::{RequestBuilder, TestClient};
use nsuns_server::{
    movements::model::{CreateMovement, Movement},
    profiles::model::{CreateProfile, Profile},
    program::model::{CreateProgram, ProgramMeta, ProgramSummary},
    router::{MOVEMENTS_PATH, PROFILES_PATH, PROGRAMS_PATH, SETS_PATH},
    sets::model::{CreateSet, Day, Set},
};
use serde::Serialize;
use serial_test::serial;
use testcontainers::clients::Cli;

trait JsonBody {
    fn json_body<T>(self, body: &T) -> Self
    where
        T: ?Sized + Serialize;
}

impl JsonBody for RequestBuilder {
    fn json_body<T>(self, body: &T) -> Self
    where
        T: ?Sized + Serialize,
    {
        self.body(serde_json::to_string(body).unwrap())
            .header("Content-Type", "application/json")
    }
}

#[tokio::test(flavor = "multi_thread")]
#[serial]
async fn create_program() {
    let docker = Cli::docker();
    let env = setup::init(&docker).await;
    let client = TestClient::new(env.router);

    // create a profile
    let create_profile = CreateProfile {
        name: "Test".into(),
    };

    let profile_id = client
        .post(PROFILES_PATH)
        .json_body(&create_profile)
        .send()
        .await
        .json::<Profile>()
        .await
        .id;

    // create a movement
    let create_bench_press = CreateMovement {
        name: "Bench Press".to_string(),
        description: None,
    };

    let bench_id = client
        .post(MOVEMENTS_PATH)
        .json_body(&create_bench_press)
        .send()
        .await
        .json::<Movement>()
        .await
        .id;

    // create an empty program
    let create_program = CreateProgram {
        description: None,
        name: "Test".to_string(),
        owner: profile_id,
    };

    let program_meta = client
        .post(PROGRAMS_PATH)
        .json_body(&create_program)
        .send()
        .await
        .json::<ProgramMeta>()
        .await;

    // add a set to the program
    let bench_set = CreateSet {
        amount: 70.0,
        day: Day::Monday,
        description: None,
        movement_id: bench_id,
        program_id: program_meta.id,
        percentage_of_max: Some(bench_id),
        reps: Some(10),
        reps_is_minimum: false,
    };

    let bench_set = client
        .post(SETS_PATH)
        .json_body(&bench_set)
        .send()
        .await
        .json::<Set>()
        .await;

    // check that the program summary includes the set
    let summary = client
        .get(&format!("{PROGRAMS_PATH}/{id}", id = program_meta.id))
        .send()
        .await
        .json::<ProgramSummary>()
        .await;

    assert_eq!(vec![bench_set], summary.sets_monday);
    assert!(
        summary.sets_sunday.is_empty(),
        "{:?} is not empty",
        summary.sets_sunday
    );
    assert_eq!(program_meta, summary.program);
}
