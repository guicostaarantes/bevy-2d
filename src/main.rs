mod plugins;
mod scenes;

use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::plugins::animation::AnimationPlugin;
use crate::plugins::camera::CameraPlugin;
use crate::plugins::collision::CollisionPlugin;
use crate::plugins::controlling::ControllingPlugin;
use crate::plugins::movement::MovementPlugin;

use crate::scenes::scene_1;
use crate::scenes::AppState;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        fit_canvas_to_parent: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_state::<AppState>()
        .add_plugins(WorldInspectorPlugin::new().run_if(input_toggle_active(false, KeyCode::Grave)))
        .add_plugins(MovementPlugin::new())
        .add_plugins(AnimationPlugin::new())
        .add_plugins(ControllingPlugin::new())
        .add_plugins(CameraPlugin::new())
        .add_plugins(CollisionPlugin::new())
        .add_systems(OnEnter(AppState::LoadingEntitiesScene1), scene_1)
        .run();
}
