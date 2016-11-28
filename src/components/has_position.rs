use custom_container::*;
use ecs::{PostUpdater, UpdatesSystem, System};

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

//impl<C, S: IsSystem<C>, E: ContainsMutSystem, U: PostUpdater<C, S, E>> UpdatesSystem for PositionUpdater {
//    fn update(&mut self, system: &C, ecs: &E, dt: f64) -> U;
//}
/*
pub struct PositionPostUpdater {
    pub new_position: Vector
}

impl PostUpdater<HasPosition, System<HasPosition>, EcsContainer> for PositionPostUpdater {
    fn post_update(self, system: &mut S, ecs: &mut E) {
        system.position = self.new_position;
    }
}
*/
