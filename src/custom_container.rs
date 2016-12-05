use ::ecs::*;
use ::components::*;

// Creates EcsContainer struct with custom fields for systems and updaters
create_container!(
    // Give it a name
    name: EcsContainer,    
    // Will add a field for each system. Syntax is 'field_name => SystemType = ComponentType'
    // Systems must implement IsSystem<Component> for their component type
    with_systems {
        has_name     => System<HasName> = HasName,
        has_health   => System<HasHealth> = HasHealth,
        has_position => System<HasPosition> = HasPosition
    },
    // Adds fields for updaters. Syntax is 'field_name => UpdaterType'
    // Updaters will be called when EcsContainer.updater(dt: f64) is called
    with_updaters {
         position_updater => PositionUpdater
    }
);
