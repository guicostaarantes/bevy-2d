use bevy::prelude::*;

use crate::plugins::movement::components::*;

pub fn calculate_position_of_entities_that_are_able_to_move(
    mut movables: Query<(&mut HasPosition, &AbleToMove)>,
    time: Res<Time>,
) {
    movables.iter_mut().for_each(|(mut pos, movable)| {
        pos.x += movable.current_speed.x * time.delta_seconds();
        pos.y += movable.current_speed.y * time.delta_seconds();
    });
}
