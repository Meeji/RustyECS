#[macro_use]
extern crate something;

use something::ecs::*;
use something::components::*;
use something::custom_container::{EcsContainer, ConfiguresComponent};

fn main() {
    let mut container = EcsContainer {
        has_name: System::new(),
        has_health: System::new(),
        has_position: System::new(),
        position_updater: PositionUpdater::new(Vector::new(5.0, 5.0)),
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
