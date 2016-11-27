pub trait FromEcs<E: ContainsSystem> where Self: Sized {
    fn from_ecs(ecs: &E) -> &Self;
}

pub trait FromEcsMut<E: ContainsMutSystem>: FromEcs<E> {
    fn from_ecs_mut(ecs: &mut E) -> &mut Self;
}

pub trait ContainsSystem where Self: Sized {
    fn get_system<S>(&self) -> &S
        where S: FromEcs<Self>;
}

pub trait ContainsMutSystem: ContainsSystem {
    fn get_system_mut<S>(&mut self) -> &mut S
        where S: FromEcsMut<Self>;
}


//#[macro_export]
macro_rules! create_container {
    (with_systems {
        $($sys_id:ident => $sys_type:ty = $cmp_type:ty),+
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

        impl<'a, F: CreatesEntities> ConfiguresComponent<$cmp_type> for EntityConfiguration<'a, EcsContainer<F>> {
            fn with_component(self, component: $cmp_type) -> Self {
                self.with_component::<$cmp_type, $sys_type>(component)
            }
        }
    )+)
}
