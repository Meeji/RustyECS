use ::ecs::*;
use ::components::*;

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
