use nsuns_server::movements::model::Movement;
use uuid::Uuid;

#[derive(Debug, Default)]
pub struct MovementWorld {
    pub movement: Option<Movement>,
    pub movements: Vec<Movement>,
}

impl MovementWorld {
    pub fn unwrap_movement(&self) -> &Movement {
        self.movement
            .as_ref()
            .expect("No movement injected into global state")
    }

    pub fn movement_by_name(&self, name: &str) -> Option<&Movement> {
        self.movements.iter().find(|movement| movement.name == name)
    }

    pub fn movement_by_id(&self, id: Uuid) -> Option<&Movement> {
        self.movements.iter().find(|movement| movement.id == id)
    }
}
