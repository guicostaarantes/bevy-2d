use bevy::prelude::*;
use bevy::time::Stopwatch;

use crate::plugins::controlling::components::*;
use crate::plugins::movement::components::*;

pub fn control_entities_that_are_able_to_jump_and_able_to_control(
    mut jumpables: Query<(&mut AbleToJump, &AbleToControl)>,
    input: Res<Input<KeyCode>>,
) {
    jumpables
        .iter_mut()
        .for_each(|(mut jumpable, controllable)| {
            if controllable.controlling
                && input.just_pressed(controllable.key_jump)
                && jumpable.jump_started == None
            {
                jumpable.jump_started = Some(Stopwatch::new());
            }
        })
}
