use std::fmt::Debug;

use axum_test_helper::TestClient;
use cucumber::World;

use crate::{
    common, maxes::world::MaxesWorld, movement::world::MovementWorld, profile::world::ProfileWorld,
    program::world::ProgramWorld,
};

#[derive(World)]
#[world(init = Self::new)]
pub struct NsunsWorld {
    pub auth_cookie: Option<String>,
    pub client: TestClient,
    pub profile_world: ProfileWorld,
    pub movement_world: MovementWorld,
    pub program_world: ProgramWorld,
    pub maxes_world: MaxesWorld,
}

impl NsunsWorld {
    async fn new() -> Self {
        let router = common::init().await;
        Self {
            auth_cookie: None,
            client: TestClient::new(router).await,
            profile_world: Default::default(),
            movement_world: Default::default(),
            program_world: Default::default(),
            maxes_world: Default::default(),
        }
    }
}

impl Debug for NsunsWorld {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NsunsWorld")
            .field("auth_cookie", &self.auth_cookie)
            .field("client", &"anonymous-client")
            .field("profile_world", &self.profile_world)
            .field("movement_world", &self.movement_world)
            .field("program_world", &self.program_world)
            .field("maxes_world", &self.maxes_world)
            .finish()
    }
}
