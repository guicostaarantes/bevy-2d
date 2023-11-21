use bevy::prelude::*;

use crate::plugins::controlling::components::*;
use crate::plugins::movement::components::*;

pub fn control_entities_that_are_able_to_move_and_able_to_control(
    mut movables: Query<(&mut HasPosition, &mut AbleToMove, &AbleToControl)>,
    input: Res<Input<KeyCode>>,
) {
    movables
        .iter_mut()
        .for_each(|(mut pos, mut movable, controllable)| {
            if controllable.controlling {
                let mut direction = Vec2::new(0., 0.);

                if input.pressed(controllable.key_up) {
                    direction.y += 1.;
                }
                if input.pressed(controllable.key_left) {
                    direction.x -= 1.;
                }
                if input.pressed(controllable.key_down) {
                    direction.y -= 1.;
                }
                if input.pressed(controllable.key_right) {
                    direction.x += 1.;
                }

                let direction_vector = direction.normalize_or_zero();
                movable.current_speed.x = movable.walk_speed * direction_vector.x;
                movable.current_speed.y = movable.walk_speed * direction_vector.y;

                if direction_vector.x != 0. || direction_vector.y != 0. {
                    pos.dir = direction_vector.into();
                }
            }
        })
}
