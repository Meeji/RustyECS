#[macro_use]
extern crate something;

use something::ecs::*;
use something::components::*;

macro_rules! create_container {
    (with_systems {
        $($sys_id:ident => $sys_type:ty = $cmp_type:ty),+
    }) => (

    pub trait ConfiguresComponent<C> {
        fn with_component(self, component: C) -> Self;
    }

    pub struct EcsContainer {
        entity_factory: EntityFactory,
        $(pub $sys_id: $sys_type,)+
    }

    impl EcsContainer {
        pub fn new_entity(&mut self) -> EntityConfiguration<Self> {
            let entity = self.entity_factory.new_entity();
            self.configure_entity(entity)
        }

        pub fn configure_entity(&mut self, entity: Entity) -> EntityConfiguration<Self> {
            EntityConfiguration::new(self, entity)
        }
    }

    impl ContainsSystem for EcsContainer {
        fn get_system<S>(&self) -> &S
            where S: FromEcs<Self> {
                FromEcs::from_ecs(self)
            }
    }

    impl ContainsMutSystem for EcsContainer {
        fn get_system_mut<S>(&mut self) -> &mut S
            where S: FromEcsMut<Self> {
                FromEcsMut::from_ecs_mut(self)
            }
    }

    $(
        impl FromEcs<EcsContainer> for $sys_type {
            fn from_ecs(ecs: &EcsContainer) -> &$sys_type {
                &ecs.$sys_id
            }
        }

        impl FromEcsMut<EcsContainer> for $sys_type {
            fn from_ecs_mut(ecs: &mut EcsContainer) -> &mut $sys_type {
                &mut ecs.$sys_id
            }
        }

        impl<'a> ConfiguresComponent<$cmp_type> for EntityConfiguration<'a, EcsContainer> {
            fn with_component(self, component: $cmp_type) -> Self {
                self.with_component_for_system::<$cmp_type, $sys_type>(component)
            }
        }
    )+)
}

create_container!(
    with_systems {
        has_name   => System<HasName> = HasName,
        has_health => System<HasHealth> = HasHealth
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
        .with_component(HasName::new("Geralt"))
        .with_component(HasHealth::new(100))
        .into();

    // Print Geralt's ID.
    println!("Geralt ID: {:?}", geralt.get_id());

    // Make sure Geralt is retrievable from system
    println!("{:?} - {:?}, {:?}",
        container.has_name.has_component(&geralt),
        container.has_name.get_component(&geralt).unwrap().name,
        container.has_health.get_component(&geralt).unwrap().health);
}
