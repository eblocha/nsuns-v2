use cucumber::World;

mod common;
mod maxes;
mod movement;
mod profile;
mod program;
mod reps;
mod sets;
mod updates;
mod util;
mod world;

#[tokio::main]
async fn main() {
    world::NsunsWorld::cucumber()
        .fail_on_skipped()
        .run_and_exit("tests/features")
        .await;
}
