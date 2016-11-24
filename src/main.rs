#[macro_use]
extern crate something;

use std::ops::{Deref, DerefMut};
use something::ecs::*;
use something::components::*;

create_container!(
    with_systems {
        has_name => System<HasName>
    }
);

fn main() {
    let mut container = EcsContainer {
        has_name: System::new(),
        entity_factory: EntityFactory::new()
     };

    // Create entity, system and ECS container
    let geralt = container.new_entity();


    // Association component with entity in system
    container.deref_mut().add_entity(&geralt, HasName::new("Geralt"));

    // Print Geralt's ID.
    println!("Geralt ID: {:?}", geralt.get_id());

    // Make sure Geralt is retrievable from system
    println!("{:?} - {:?}",
        container.has_name.has_component(&geralt),
        container.has_name.get_component(&geralt).unwrap().name);
}
