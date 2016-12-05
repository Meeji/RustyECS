use std::collections::HashMap;
use ecs::entity::{Entity, EntityHashState};
use ecs::container::ContainsMutSystem;

pub trait HasSystemContainer {
    type container: ContainsMutSystem;

    fn get_systems(&self) -> &Self::container;

    fn get_systems_mut(&mut self) -> &mut Self::container;
}

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

pub trait UpdatesEcs<E: HasSystemContainer> {
    fn update(&self, ecs: &mut E::container, dt: f64);
}

pub struct System<C> {
    entity_components: HashMap<Entity, C, EntityHashState>
}

impl<C> System<C> {
    pub fn new() -> System<C> {
        System {
            entity_components: HashMap::with_hasher(EntityHashState),
        }
    }
}

impl<C> IsSystem<C> for System<C> {
    fn has_component(&self, entity: &Entity) -> bool {
        self.entity_components.contains_key(entity)
    }

    fn add_entity(&mut self, entity: &Entity, component: C) -> bool {
        if self.entity_components.contains_key(entity) {
            false
        } else {
            self.entity_components.insert(*entity, component);
            true
        }
    }

    fn get_component(&self, entity: &Entity) -> Option<&C> {
        self.entity_components.get(entity)
    }

    fn get_component_mut(&mut self, entity: &Entity) -> Option<&mut C> {
        self.entity_components.get_mut(entity)
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
        self.entity_components.contains_key(entity)
    }

    fn remove_entity(&mut self, entity: &Entity) -> bool {
        if self.entity_components.contains_key(entity) {
            self.entity_components.remove(entity);
            true
        } else { false }
    }
}
