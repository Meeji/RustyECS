# RustyECS

This is a port of the simple C# ECS. I'm hoping to preserve the fluentness of the API and teach myself some Rust and some more about ECSes in the process. The (very early and not at all finalised) API looks like:

```rust
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
```
