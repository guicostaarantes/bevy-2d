use bevy::prelude::*;

use crate::plugins::camera::components::*;
use crate::plugins::movement::components::*;

const CAMERA_POSITION_THRESHOLD: f32 = 0.01;

pub fn follow_entity_that_is_followed_by_camera(
    mut camera: Query<&mut Transform, With<OrthographicProjection>>,
    followables: Query<(&HasPosition, &IsFollowedByCamera)>,
) {
    let mut cam = camera.single_mut();

    followables.iter().for_each(|(pos, followable)| {
        if followable.active {
            if (cam.translation.x - pos.x).abs() > CAMERA_POSITION_THRESHOLD
                || (cam.translation.y - pos.y).abs() > CAMERA_POSITION_THRESHOLD
            {
                cam.translation.x =
                    (followable.damping * cam.translation.x + pos.x) / (followable.damping + 1.);
                cam.translation.y =
                    (followable.damping * cam.translation.y + pos.y) / (followable.damping + 1.);
            }
        }
    })
}
