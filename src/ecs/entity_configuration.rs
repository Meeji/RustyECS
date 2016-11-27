use ecs::*;

pub struct EntityConfiguration<'a, E: 'a + ContainsMutSystem> {
    container: &'a mut E,
    entity: Entity
}

impl<'a, E: 'a + ContainsMutSystem> EntityConfiguration<'a, E> {
    pub fn new(container: &'a mut E, entity: Entity) -> EntityConfiguration<'a, E> {
        EntityConfiguration { container: container, entity: entity }
    }

    pub fn with_component_for_system<C, S: FromEcsMut<E> + IsSystem<C>>(mut self, component: C) -> Self {
        self.container.get_system_mut::<S>().add_entity(&self.entity, component);
        self
    }
}

impl<'a, E: 'a + ContainsMutSystem> Into<Entity> for EntityConfiguration<'a, E> {
    fn into(self) -> Entity { self.entity }
}
