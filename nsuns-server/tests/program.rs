mod common;

use axum_test_helper::TestClient;
use nsuns_server::{
    program::model::{ProgramSummary, ReorderSets},
    router::{PROGRAMS_PATH, SETS_PATH},
    sets::model::{CreateSet, Day, Set},
};

use common::util::JsonBody;
use common::setup::{InitialProgramState, setup_program_state};
use uuid::Uuid;

async fn get_summary(client: &TestClient, id: Uuid) -> ProgramSummary {
    client
        .get(&format!("{PROGRAMS_PATH}/{id}"))
        .send()
        .await
        .json::<ProgramSummary>()
        .await
}

#[tokio::test(flavor = "multi_thread")]
async fn create_program() {
    let router = common::setup::init().await;
    let client = TestClient::new(router);

    let InitialProgramState {
        program_meta,
        movement,
    } = setup_program_state(&client).await;

    // add a set to the program
    let bench_set = CreateSet {
        amount: 70.0,
        day: Day::Monday,
        description: None,
        movement_id: movement.id,
        program_id: program_meta.id,
        percentage_of_max: Some(movement.id),
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
    let summary = get_summary(&client, program_meta.id).await;

    assert_eq!(vec![bench_set], summary.sets_monday);
    assert!(
        summary.sets_sunday.is_empty(),
        "{:?} is not empty",
        summary.sets_sunday
    );
    assert_eq!(program_meta, summary.program);
}

#[tokio::test(flavor = "multi_thread")]
async fn reorder_sets() {
    let router = common::setup::init().await;
    let client = TestClient::new(router);

    let InitialProgramState {
        program_meta,
        movement,
    } = setup_program_state(&client).await;

    // add 2 sets to the program
    let bench_set = CreateSet {
        amount: 70.0,
        day: Day::Monday,
        description: None,
        movement_id: movement.id,
        program_id: program_meta.id,
        percentage_of_max: Some(movement.id),
        reps: Some(10),
        reps_is_minimum: false,
    };

    let bench_set_2 = CreateSet {
        amount: 70.0,
        day: Day::Monday,
        description: None,
        movement_id: movement.id,
        program_id: program_meta.id,
        percentage_of_max: Some(movement.id),
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

    let bench_set_2 = client
        .post(SETS_PATH)
        .json_body(&bench_set_2)
        .send()
        .await
        .json::<Set>()
        .await;

    // reorder the set
    let reordered = client
        .post(&format!("{PROGRAMS_PATH}/reorder-sets"))
        .json_body::<ReorderSets>(&ReorderSets {
            program_id: program_meta.id,
            day: Day::Monday,
            from: 0,
            to: 1,
        })
        .send()
        .await
        .json::<Vec<Uuid>>()
        .await;

    // check that the program summary has the correct order
    let summary = get_summary(&client, program_meta.id).await;

    assert_eq!(
        vec![bench_set_2.clone(), bench_set.clone()],
        summary.sets_monday
    );

    assert_eq!(vec![bench_set_2.id, bench_set.id], reordered);
}
