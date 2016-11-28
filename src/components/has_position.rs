use custom_container::*;
use ecs::{Entity, PostUpdater, UpdatesSystem, System, IsSystem};

pub struct Vector {
    pub x: f64,
    pub y: f64
}

impl Vector {
    pub fn new(x: f64, y: f64) -> Vector {
        Vector { x: x, y : y}
    }
}

pub struct HasPosition {
    pub position: Vector
}

impl HasPosition {
    pub fn new(position: Vector) -> HasPosition {
        HasPosition { position: position }
    }
}

pub struct PositionUpdater {
    pub movement: Vector
}

impl PositionUpdater {
    pub fn new(movement: Vector) -> PositionUpdater {
        PositionUpdater { movement: movement }
    }
}

impl UpdatesSystem<HasPosition, System<HasPosition>, EcsContainer, PositionPostUpdater> for PositionUpdater {
    fn update(&mut self, _: &System<HasPosition>, _: &EcsContainer, _: f64) -> PositionPostUpdater {
        PositionPostUpdater {
            new_position: (Entity::new(1), Vector::new(5.0, 5.0))
        }
    }
}

pub struct PositionPostUpdater {
    pub new_position: (Entity, Vector)
}

impl PostUpdater<HasPosition, System<HasPosition>, EcsContainer> for PositionPostUpdater {
    fn post_update(self, system: &mut System<HasPosition>, _: &mut EcsContainer) {
        system.get_component_mut(&self.new_position.0).unwrap().position = self.new_position.1;
    }
}
