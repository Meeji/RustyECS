use custom_container::*;
use ecs::{Entity, UpdatesEcs, System, IsSystem, ContainsMutSystem};

#[derive(Debug)]
pub struct Vector {
    pub x: f64,
    pub y: f64
}

impl Vector {
    pub fn new(x: f64, y: f64) -> Vector {
        Vector { x: x, y : y}
    }
}

#[derive(Debug)]
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

impl UpdatesEcs<EcsContainer> for PositionUpdater {
    fn update(&self, ecs: &mut EcsContainer, dt: f64) {
        let system = ecs.get_system_mut::<System<HasPosition>>();
        system.get_component_mut(&Entity::new(1)).unwrap().position = Vector::new(dt, dt);
    }
}
