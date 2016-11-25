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
        pub fn new_entity(&mut self) -> EntityConfiguration<Self> {
            let entity = self.entity_factory.new_entity();
            self.configure_entity(entity)
        }

        pub fn configure_entity(&mut self, entity: Entity) -> EntityConfiguration<Self> {
            EntityConfiguration::new(self, entity)
        }
    }

    impl<F: CreatesEntities> ContainsSystem for EcsContainer<F> {
        fn get_system<S>(&self) -> &S
            where S: FromEcs<Self> {
                FromEcs::from_ecs(self)
            }
    }

    impl<F : CreatesEntities> ContainsMutSystem for EcsContainer<F> {
        fn get_system_mut<S>(&mut self) -> &mut S
            where S: FromEcsMut<Self> {
                FromEcsMut::from_ecs_mut(self)
            }
    }

    $(
        impl<F: CreatesEntities> FromEcs<EcsContainer<F>> for $sys_type {
            fn from_ecs(ecs: &EcsContainer<F>) -> &$sys_type {
                &ecs.$sys_id
            }
        }

        impl<F : CreatesEntities> FromEcsMut<EcsContainer<F>> for $sys_type {
            fn from_ecs_mut(ecs: &mut EcsContainer<F>) -> &mut $sys_type {
                &mut ecs.$sys_id
            }
        }
    )+)
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

    // Create entity, and configure components
    let geralt: Entity = container.new_entity()
        .with_component::<HasName, System<HasName>>(HasName::new("Geralt"))
        .with_component::<HasHealth, System<HasHealth>>(HasHealth::new(100))
        .into();

    // Print Geralt's ID.
    println!("Geralt ID: {:?}", geralt.get_id());

    // Make sure Geralt is retrievable from system
    println!("{:?} - {:?}",
        container.has_name.has_component(&geralt),
        container.has_name.get_component(&geralt).unwrap().name);
}
