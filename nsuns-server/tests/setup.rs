use std::collections::HashMap;

use axum::Router;
use nsuns_server::{
    db::{default_max_connections, default_timeout, DatabaseSettings},
    metrics::settings::MetricsFeature,
    openapi::settings::OpenApiFeature,
    server,
    settings::{ServerSettings, Settings},
};
use testcontainers::{clients::Cli, core::WaitFor, Container, Image};

#[derive(Debug)]
pub struct Postgres {
    env_vars: HashMap<String, String>,
}

impl Default for Postgres {
    fn default() -> Self {
        let env_vars = HashMap::new();
        Self { env_vars }
    }
}

impl Postgres {
    pub fn with_username(mut self, username: String) -> Self {
        self.env_vars.insert("POSTGRES_USER".to_string(), username);
        self
    }

    pub fn with_password(mut self, password: String) -> Self {
        self.env_vars
            .insert("POSTGRES_PASSWORD".to_string(), password);
        self
    }

    pub fn with_database(mut self, database: String) -> Self {
        self.env_vars.insert("POSTGRES_DB".to_owned(), database);
        self
    }
}

impl Image for Postgres {
    type Args = ();

    fn name(&self) -> String {
        "postgres".to_owned()
    }

    fn tag(&self) -> String {
        "latest".to_owned()
    }

    fn ready_conditions(&self) -> Vec<WaitFor> {
        vec![WaitFor::message_on_stderr(
            "database system is ready to accept connections",
        )]
    }

    fn env_vars(&self) -> Box<dyn Iterator<Item = (&String, &String)> + '_> {
        Box::new(self.env_vars.iter())
    }
}

pub struct DatabaseAndSettings<'d> {
    container: Container<'d, Postgres>,
    settings: DatabaseSettings,
}

pub fn setup_postgres(client: &Cli) -> DatabaseAndSettings<'_> {
    let database = "postgres";
    let username = "postgres";
    let password = "postgres";

    let container = client.run(
        Postgres::default()
            .with_database(database.to_string())
            .with_password(password.to_string())
            .with_username(username.to_string()),
    );

    let port = container
        .ports()
        .map_to_host_port_ipv4(5432)
        .expect("failed to get database port");

    DatabaseAndSettings {
        settings: DatabaseSettings {
            database: database.to_string(),
            host: "localhost".to_string(),
            password: password.to_string(),
            username: username.to_string(),
            port,
            migrations: "db/migrations".to_string(),
            max_connections: default_max_connections(),
            timeout: default_timeout(),
        },
        container,
    }
}

pub struct Environment<'c> {
    pub db_container: Container<'c, Postgres>,
    pub router: Router,
}

pub async fn init<'a>(docker: &'a Cli) -> Environment<'a> {
    let DatabaseAndSettings {
        container,
        settings,
    } = setup_postgres(docker);

    let router = server::initialize(&Settings {
        server: ServerSettings {
            port: 0,
            static_dir: None,
        },
        database: settings,
        metrics: MetricsFeature::Disabled,
        openapi: OpenApiFeature::Disabled,
    })
    .await
    .unwrap();

    Environment {
        db_container: container,
        router,
    }
}
