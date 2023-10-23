pub mod components;
mod systems;

use bevy::prelude::*;

use self::components::IsFollowedByCamera;
use self::systems::follow_entity_that_is_followed_by_camera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, follow_entity_that_is_followed_by_camera)
            .register_type::<IsFollowedByCamera>();
    }
}

impl CameraPlugin {
    pub fn new() -> Self {
        Self {}
    }
}
