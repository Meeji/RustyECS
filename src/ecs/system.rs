use std::collections::{HashMap, HashSet};
use ecs::entity::{Entity, EntityHashState};
use ecs::container::ContainsMutSystem;

pub trait AssociatesEntities {
    fn has_entity(&self, entity: &Entity) -> bool;

    fn remove_entity(&mut self, entity: &Entity) -> bool;
}

pub trait HasSystem<C, S: IsSystem<C>> {
    fn get_system(&self) -> &S;

    fn get_system_mut(&mut self) -> &mut S;
}

pub trait IsSystem<C>: AssociatesEntities {
    fn add_entity(&mut self, entity: &Entity, component: C) -> bool;

    fn has_component(&self, entity: &Entity) -> bool;

    fn get_component(&self, entity: &Entity) -> Option<&C>;

    fn get_component_mut(&mut self, entity: &Entity) -> Option<&mut C>;
}

pub trait UpdatesEcs<E: ContainsMutSystem> {
    fn update(&self, ecs: &mut E, dt: f64);
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

    fn get_component_mut(&mut self, entity: &Entity) -> Option<&mut C> {
        self.map.get_mut(entity)
    }
}

impl<C> HasSystem<C, System<C>> for System<C> {
    fn get_system(&self) -> &System<C> {
        self
    }

    fn get_system_mut(&mut self) -> &mut System<C> {
        self
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
