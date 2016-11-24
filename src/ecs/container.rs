/*
create_container(
    with_systems {
        has_name_key   => has_name,
        has_health_key => has_health,
    }
)
*/

#[macro_export]
macro_rules! create_container {
    (with_systems {
        $($sys_id:ident => $sys_type:ty),+
    }) => (
    // let mut __identifier = 444usize;

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
    )+)
}
