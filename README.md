# RustyECS

This is a port of the simple C# ECS. I'm hoping to preserve the fluentness of the API and teach myself some Rust and some more about ECSes in the process. The (very early and not at all finalised) API looks like:

```rust
// Creates EcsContainer struct with custom fields for systems and updaters
create_container!(
    name: EcsContainer,
    with_systems {
        has_name     => System<HasName> = HasName,
        has_health   => System<HasHealth> = HasHealth,
        has_position => System<HasPosition> = HasPosition
    },
    with_updaters {
         position_updater => PositionUpdater
    }
);

fn main() {
    // Create container
    let mut container = EcsContainer {
        systems: SystemContainer {
            has_name: System::new(),
            has_health: System::new(),
            has_position: System::new()
        },
        updaters: UpdaterContainer {
            position_updater: PositionUpdater::new()
        },
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
        container.systems.has_name.has_component(&geralt));

    // Retrieve some data about Geralt
    println!("name: {:?}, health: {:?}, position: {:?}",
        container.systems.has_name.get_component(&geralt).unwrap().name,
        container.systems.has_health.get_component(&geralt).unwrap().health,
        container.systems.has_position.get_component(&geralt).unwrap().position);

    // Update the container, providing a step value
    container.update(2.0);

    // Check data has updated
    println!("new position {:?}",
        container.systems.has_position.get_component(&geralt).unwrap().position);
}
```
