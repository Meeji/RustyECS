use std::ops::Add;
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

impl Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector::new(self.x + other.x, self.y + other.y)
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

pub struct PositionUpdater;

impl PositionUpdater {
    pub fn new() -> PositionUpdater { PositionUpdater }
}

impl UpdatesEcs<EcsContainer> for PositionUpdater {
    fn update(&self, ecs: &mut EcsContainer, dt: f64) {
impl Default for PositionUpdater {
    fn default() -> PositionUpdater { PositionUpdater::new() }
}

impl UpdatesSystem<HasPosition, System<HasPosition>, EcsContainer, PositionPostUpdater> for PositionUpdater {
    fn update(&self, _: &System<HasPosition>, _: &EcsContainer, dt: f64) -> PositionPostUpdater {
        PositionPostUpdater {
            new_position: (Entity::new(1), Vector::new(dt, dt))
        }
    }
}

pub struct PositionPostUpdater {
    pub new_position: (Entity, Vector)
}

        let system = ecs.get_system_mut::<System<HasPosition>>();
        system.get_component_mut(&Entity::new(1)).unwrap().position = Vector::new(dt, dt);
    }
}
