use axum::Router;
use nsuns_server::{
    db::{default_timeout, DatabaseSettings},
    metrics::settings::MetricsFeature,
    openapi::settings::OpenApiFeature,
    server,
    settings::{ServerSettings, Settings},
};
use sqlx::Connection;
use uuid::Uuid;

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

    conn.close().await?;

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
            // try to limit connection pressure on test DB
            max_connections: 1,
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
