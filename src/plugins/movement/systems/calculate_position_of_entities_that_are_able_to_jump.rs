use bevy::prelude::*;

use crate::plugins::movement::components::*;

pub fn calculate_position_of_entities_that_are_able_to_jump(
    mut jumpables: Query<(&mut HasPosition, &mut AbleToJump)>,
    time: Res<Time>,
) {
    jumpables
        .iter_mut()
        .for_each(|(mut pos, mut jumpable)| match &mut jumpable.jump_started {
            Some(sw) => {
                sw.tick(time.delta());
                let elapsed = sw.elapsed_secs();
                let jump_current_height = 1000.0 * elapsed * (jumpable.jump_duration - elapsed);
                if jump_current_height < 0.0 {
                    jumpable.jump_started = None;
                    pos.z = 0.0;
                } else {
                    pos.z = jump_current_height;
                }
            }
            None => {}
        })
}
