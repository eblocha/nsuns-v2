use axum::Router;
use axum_test_helper::TestClient;
use nsuns_server::{
    db::{default_max_connections, default_timeout, DatabaseSettings},
    metrics::settings::MetricsFeature,
    movements::model::{CreateMovement, Movement},
    openapi::settings::OpenApiFeature,
    profiles::model::{CreateProfile, Profile},
    program::model::{CreateProgram, ProgramMeta},
    router::{MOVEMENTS_PATH, PROFILES_PATH, PROGRAMS_PATH},
    server,
    settings::{ServerSettings, Settings},
};
use sqlx::Connection;
use uuid::Uuid;

use super::util::JsonBody;

/// Create a randomized DB to re-use the container for multiple tests concurrently
async fn randomize_db(mut settings: DatabaseSettings) -> anyhow::Result<DatabaseSettings> {
    let mut conn = sqlx::PgConnection::connect(&format!(
        "postgres://{}:{}@{}:{}/{}",
        settings.username, settings.password, settings.host, settings.port, settings.database
    ))
    .await?;

    let database = format!("db_{}", Uuid::new_v4().to_string().replace("-", "_"));

    sqlx::query(&format!("CREATE DATABASE {database};"))
        .execute(&mut conn)
        .await?;

    settings.database = database;

    Ok(settings)
}

pub async fn init() -> Router {
    server::initialize(&Settings {
        server: ServerSettings {
            port: 0,
            static_dir: None,
        },
        database: randomize_db(DatabaseSettings {
            database: "postgres".to_string(),
            host: "localhost".to_string(),
            password: "postgres".to_string(),
            username: "postgres".to_string(),
            port: 5433,
            migrations: "db/migrations".to_string(),
            max_connections: default_max_connections(),
            timeout: default_timeout(),
        })
        .await
        .unwrap(),
        metrics: MetricsFeature::Disabled,
        openapi: OpenApiFeature::Disabled,
    })
    .await
    .unwrap()
}

pub struct InitialProgramState {
    pub program_meta: ProgramMeta,
    pub movement: Movement,
}

#[allow(dead_code)] // see https://github.com/rust-lang/rust/issues/46379
pub async fn setup_profile(client: &TestClient) -> Profile {
    let create_profile = CreateProfile {
        name: "Test".into(),
    };

    client
        .post(PROFILES_PATH)
        .json_body(&create_profile)
        .send()
        .await
        .json::<Profile>()
        .await
}

#[allow(dead_code)] // see https://github.com/rust-lang/rust/issues/46379
pub async fn setup_program_state(client: &TestClient) -> InitialProgramState {
    let profile_id = setup_profile(client).await.id;

    // create a movement
    let create_bench_press = CreateMovement {
        name: "Bench Press".to_string(),
        description: None,
    };

    let movement = client
        .post(MOVEMENTS_PATH)
        .json_body(&create_bench_press)
        .send()
        .await
        .json::<Movement>()
        .await;

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

    InitialProgramState {
        program_meta,
        movement,
    }
}
