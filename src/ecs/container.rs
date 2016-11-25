/*
create_container(
    with_systems {
        has_name_key   => has_name,
        has_health_key => has_health,
    }
)


*/

pub trait FromEcs<'a, E: ContainsSystem> where Self: Sized {
    fn from_ecs(ecs: &'a E) -> &'a Self;
}

pub trait FromEcsMut<'a, E: ContainsMutSystem>: FromEcs<'a, E> {
    fn from_ecs_mut(ecs: &'a mut E) -> &'a mut Self;
}

pub trait ContainsSystem where Self: Sized {
    fn get_system<'a, S>(&'a self) -> &'a S
        where S: FromEcs<'a, Self>;
}

pub trait ContainsMutSystem: ContainsSystem {
    fn get_system_mut<'a, S>(&'a mut self) -> &'a mut S
        where S: FromEcsMut<'a, Self>;
}


// #[macro_export]
macro_rules! create_container {
    (with_systems {
        $($sys_id:ident => $sys_type:ty),+
    }) => (

    pub struct EcsContainer<F : CreatesEntities> {
        entity_factory: F,
        $(pub $sys_id: $sys_type,)+
    }

    impl<F : CreatesEntities> EcsContainer<F> {
        pub fn new_entity(&mut self) -> Entity {
            self.entity_factory.new_entity()
        }
    }

    $(
        impl<F : CreatesEntities> Deref for EcsContainer<F> {
            type Target = $sys_type;

            fn deref(&self) -> &$sys_type {
                &self.$sys_id
            }
        }

        impl<F : CreatesEntities> DerefMut for EcsContainer<F> {
            fn deref_mut(&mut self) -> &mut $sys_type {
                &mut self.$sys_id
            }
        }
    )+

    pub struct EntityConfiguration {
        entity: Entity,
    }

    impl EntityConfiguration {
        pub fn new(entity: Entity) -> EntityConfiguration {
            EntityConfiguration { entity: entity }
        }
    })
}
