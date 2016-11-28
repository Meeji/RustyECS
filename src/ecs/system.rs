use std::collections::{HashMap, HashSet};
use ecs::entity::{Entity, EntityHashState};
use ecs::container::ContainsMutSystem;


pub trait AssociatesEntities {
    fn has_entity(&self, entity: &Entity) -> bool;

    fn remove_entity(&mut self, entity: &Entity) -> bool;
}

pub trait IsSystem<C>: AssociatesEntities {
    fn add_entity(&mut self, entity: &Entity, component: C) -> bool;

    fn has_component(&self, entity: &Entity) -> bool;

    fn get_component(&self, entity: &Entity) -> Option<&C>;
}

pub trait PostUpdater<E: ContainsMutSystem> {
    fn post_update(self, ecs: &mut E);
}

pub trait IsUpdatableSystem<C, E: ContainsMutSystem, U: PostUpdater<E>>: IsSystem<C> {
    fn update(&mut self, ecs: &E, dt: f64) -> U;
}

pub struct System<C> {
    entities: HashSet<Entity, EntityHashState>,
    map: HashMap<Entity, C, EntityHashState>
}

impl<C> System<C> {
    pub fn new() -> System<C> {
        System {
            map: HashMap::with_hasher(EntityHashState),
            entities: HashSet::with_hasher(EntityHashState)
        }
    }
}

impl<C> IsSystem<C> for System<C> {
    fn has_component(&self, entity: &Entity) -> bool {
        self.map.contains_key(entity)
    }

    fn add_entity(&mut self, entity: &Entity, component: C) -> bool {
        if self.map.contains_key(entity) {
            false
        } else {
            self.map.insert(*entity, component);
            self.entities.insert(*entity);
            true
        }
    }

    fn get_component(&self, entity: &Entity) -> Option<&C> {
        self.map.get(entity)
    }
}

impl<C> AssociatesEntities for System<C> {
    fn has_entity(&self, entity: &Entity) -> bool {
        self.entities.contains(entity)
    }

    fn remove_entity(&mut self, entity: &Entity) -> bool {
        if self.entities.contains(entity) {
            self.map.remove(entity);
            self.entities.remove(entity);
            true
        } else { false }
    }
}
