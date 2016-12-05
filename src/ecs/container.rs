#![macro_use]

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

#[macro_export]
macro_rules! create_container {(
    with_systems {
        $($sys_id:ident => $sys_type:ty = $cmp_type:ty),+
    },
    with_updaters {
        $($upd_id:ident updates $upd_sys_id:ident => $upd_type:ty),+
    }
) => (
    pub trait ConfiguresComponent<C> {
        fn with_component(self, component: C) -> Self;
    }

    pub struct SystemContainer {
        $(pub $sys_id: $sys_type,)+
    }

    pub struct UpdaterContainer {
        $(pub $upd_id: $upd_type,)+
    }

    pub struct EcsContainer {
        pub entity_factory: EntityFactory,
        pub systems: SystemContainer,
        pub updaters: UpdaterContainer,
    }

    impl EcsContainer {
        pub fn new_entity(&mut self) -> EntityConfiguration<SystemContainer> {
            let entity = self.entity_factory.new_entity();
            self.configure_entity(entity)
        }

        pub fn configure_entity(&mut self, entity: Entity) -> EntityConfiguration<SystemContainer> {
            EntityConfiguration::new(&mut self.systems, entity)
        }

        pub fn update(&mut self, dt: f64) {
            $({
                let updater = &self.updaters.$upd_id;
                updater.update(&mut self.systems, dt);
            })+
        }
    }

    impl ContainsSystem for SystemContainer {
        fn get_system<S>(&self) -> &S
            where S: FromEcs<Self> {
                FromEcs::from_ecs(self)
            }
    }

    impl ContainsMutSystem for SystemContainer {
        fn get_system_mut<S>(&mut self) -> &mut S
            where S: FromEcsMut<Self> {
                FromEcsMut::from_ecs_mut(self)
            }
    }

    $(
        impl FromEcs<SystemContainer> for $sys_type {
            fn from_ecs(ecs: &SystemContainer) -> &$sys_type {
                ecs.$sys_id.get_system()
            }
        }

        impl FromEcsMut<SystemContainer> for $sys_type {
            fn from_ecs_mut(ecs: &mut SystemContainer) -> &mut $sys_type {
                ecs.$sys_id.get_system_mut()
            }
        }

        impl<'a> ConfiguresComponent<$cmp_type> for EntityConfiguration<'a, SystemContainer> {
            fn with_component(self, component: $cmp_type) -> Self {
                self.with_component_for_system::<$cmp_type, $sys_type>(component)
            }
        }
    )+

    impl HasSystemContainer for EcsContainer {
        type container = SystemContainer;

        fn get_systems(&self) -> &SystemContainer {
            &self.systems
        }

        fn get_systems_mut(&mut self) -> &mut SystemContainer {
            &mut self.systems
        }
    }
)}
