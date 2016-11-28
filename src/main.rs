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
        .with_component(HasPosition::new(Vector::new(1.0, 1.0)))
        .into();

    // Print Geralt's ID, and make sure he's in a system.
    println!("Geralt ID: {:?}: {:?}",
        geralt.get_id(),
        container.has_name.has_component(&geralt));

    // Retrieve some data about Geralt
    println!("name: {:?}, health: {:?}, position: {:?}",
        container.has_name.get_component(&geralt).unwrap().name,
        container.has_health.get_component(&geralt).unwrap().health,
        container.has_position.get_component(&geralt).unwrap().position);

    // Update the container, providing a step value
    container.update(2.0);

    // Check data has updated
    println!("new position {:?}",
        container.has_position.get_component(&geralt).unwrap().position);
}
