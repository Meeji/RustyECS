use ::ecs::*;
use ::components::*;

create_container!(
    with_systems {
        has_name   => System<HasName> = HasName,
        has_health => System<HasHealth> = HasHealth
    }
);
