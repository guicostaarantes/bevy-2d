use bevy::prelude::*;

use crate::plugins::movement::components::*;

pub fn translate_entities_that_have_position(
    mut positionables: Query<(&mut Transform, &HasPosition), Changed<HasPosition>>,
) {
    positionables.iter_mut().for_each(|(mut tf, pos)| {
        tf.translation.x = pos.x;
        tf.translation.y = pos.y + pos.z;
    })
}
