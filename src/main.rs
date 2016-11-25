#[macro_use]
extern crate something;

use something::ecs::*;
use something::components::*;

macro_rules! create_container {
    (with_systems {
        $($sys_id:ident => $sys_type:ty),+
    }) => (

    pub struct EcsContainer<F: CreatesEntities> {
        entity_factory: F,
        $(pub $sys_id: $sys_type,)+
    }

    impl<F: CreatesEntities> EcsContainer<F> {
        pub fn new_entity(&mut self) -> Entity {
            self.entity_factory.new_entity()
        }
    }

    impl<F: CreatesEntities> ContainsSystem for EcsContainer<F> {
        fn get_system<'a, S>(&'a self) -> &'a S
            where S: FromEcs<'a, Self> {
                FromEcs::from_ecs(self)
            }
    }

    impl<F : CreatesEntities> ContainsMutSystem for EcsContainer<F> {
        fn get_system_mut<'a, S>(&'a mut self) -> &'a mut S
            where S: FromEcsMut<'a, Self> {
                FromEcsMut::from_ecs_mut(self)
            }
    }

    $(
        impl<'a, F: CreatesEntities> FromEcs<'a, EcsContainer<F>> for $sys_type {
            fn from_ecs(ecs: &'a EcsContainer<F>) -> &'a $sys_type {
                &ecs.$sys_id
            }
        }

        impl<'a, F : CreatesEntities> FromEcsMut<'a, EcsContainer<F>> for $sys_type {
            fn from_ecs_mut(ecs: &'a mut EcsContainer<F>) -> &'a mut $sys_type {
                &mut ecs.$sys_id
            }
        }
    )+

    pub struct EntityConfiguration {
        entity: Entity
    }

    impl EntityConfiguration {
        pub fn new(entity: Entity) -> EntityConfiguration {
            EntityConfiguration { entity: entity }
        }
    })
}

create_container!(
    with_systems {
        has_name => System<HasName>,
        has_health => System<HasHealth>
    }
);

fn main() {
    let mut container = EcsContainer {
        has_name: System::new(),
        has_health: System::new(),
        entity_factory: EntityFactory::new()
     };

    // Create entity, system and ECS container
    let geralt = container.new_entity();

    // Association component with entity in system
    // container.get_system_mut().add_entity(&geralt, HasName::new("Geralt"));
    container.get_system_mut::<System<HasName>>().add_entity(&geralt, HasName::new("Geralt"));

    // Print Geralt's ID.
    println!("Geralt ID: {:?}", geralt.get_id());

    // Make sure Geralt is retrievable from system
    println!("{:?} - {:?}",
        container.has_name.has_component(&geralt),
        container.has_name.get_component(&geralt).unwrap().name);
}
