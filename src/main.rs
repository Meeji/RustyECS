#[macro_use]
extern crate something;

use something::ecs::*;
use something::components::*;

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
        .with_component::<_, System<_>>(HasName::new("Geralt"))
        .with_component::<_, System<_>>(HasHealth::new(100))
        .into();

    // Print Geralt's ID.
    println!("Geralt ID: {:?}", geralt.get_id());

    // Make sure Geralt is retrievable from system
    println!("{:?} - {:?}, {:?}",
        container.has_name.has_component(&geralt),
        container.has_name.get_component(&geralt).unwrap().name,
        container.has_health.get_component(&geralt).unwrap().health);
}
