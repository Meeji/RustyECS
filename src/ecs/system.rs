use std::any::Any;
use std::collections::{HashMap, HashSet};
use ecs::entity::*;


pub trait AssociatesEntities {
    fn has_entity(&self, entity: &Entity) -> bool;

    fn remove_entity(&mut self, entity: &Entity) -> bool;
}

pub trait CanBeAny {
    fn as_any(&self) -> &Any;
}

pub struct System<C> {
    entities: HashSet<Entity, EntityHashState>,
    map: HashMap<Entity, C, EntityHashState>
}
impl <C> System<C> {
    pub fn new() -> System<C> {
        System {
            map: HashMap::with_hasher(EntityHashState),
            entities: HashSet::with_hasher(EntityHashState)
        }
    }

    pub fn has_component(&self, entity: &Entity) -> bool {
        self.map.contains_key(entity)
    }

    pub fn add_entity(&mut self, entity: &Entity, component: C) -> bool {
        if self.map.contains_key(entity) {
            false
        } else {
            self.map.insert(*entity, component);
            self.entities.insert(*entity);
            true
        }
    }

    pub fn get_component(&self, entity: &Entity) -> Option<&C> {
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
impl<C> CanBeAny for System<C> where C : 'static {
    fn as_any(&self) -> &Any { self }
}
