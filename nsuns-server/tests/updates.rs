mod common;

use axum_test_helper::TestClient;
use nsuns_server::{
    maxes::model::{CreateMax, Max},
    movements::model::{CreateMovement, Movement},
    reps::model::{CreateReps, Reps},
    router::{MAXES_PATH, MOVEMENTS_PATH, REPS_PATH, UPDATES_PATH},
    updates::handler::{UpdatedState, Updates},
};

use common::setup::setup_profile;
use common::util::JsonBody;

#[tokio::test(flavor = "multi_thread")]
async fn run_updates() {
    let router = common::setup::init().await;
    let client = TestClient::new(router);

    let profile_id = setup_profile(&client).await.id;

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

    // create a latest max
    let first_max = client
        .post(MAXES_PATH)
        .json_body(&CreateMax {
            amount: 100_f64,
            movement_id: bench_id,
            profile_id,
        })
        .send()
        .await
        .json::<Max>()
        .await;

    // create a latest reps
    let first_reps = client
        .post(REPS_PATH)
        .json_body(&CreateReps {
            amount: Some(5),
            movement_id: bench_id,
            profile_id,
        })
        .send()
        .await
        .json::<Reps>()
        .await;

    // run updates
    let new_state = client
        .post(UPDATES_PATH)
        .json_body(&Updates {
            movement_ids: vec![bench_id],
            profile_id,
        })
        .send()
        .await
        .json::<UpdatedState>()
        .await;

    // check that the maxes have persisted
    let maxes = client
        .get(&format!("{MAXES_PATH}?profileId={profile_id}"))
        .send()
        .await
        .json::<Vec<Max>>()
        .await;

    // check that the reps have persisted
    let reps = client
        .get(&format!("{REPS_PATH}?profileId={profile_id}"))
        .send()
        .await
        .json::<Vec<Reps>>()
        .await;

    assert_eq!([vec![first_max], new_state.maxes].concat(), maxes);
    assert_eq!([vec![first_reps], new_state.reps].concat(), reps);
}
