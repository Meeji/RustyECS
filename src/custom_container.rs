use ::ecs::*;
use ::components::*;

create_container!(
    with_systems {
        has_name         => System<HasName> = HasName,
        has_health       => System<HasHealth> = HasHealth,
        has_position     => System<HasPosition> = HasPosition
    },
    with_updaters {
         position_updater updates has_position => PositionUpdater
    }
);
