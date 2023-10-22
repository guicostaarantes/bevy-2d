mod plugins;
mod scenes;

use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::plugins::animation::AnimationPlugin;
use crate::plugins::controlling::ControllingPlugin;
use crate::plugins::movement::MovementPlugin;

use crate::scenes::initial_scene;

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
        .add_plugins(WorldInspectorPlugin::new().run_if(input_toggle_active(false, KeyCode::Grave)))
        .add_plugins(MovementPlugin::new())
        .add_plugins(AnimationPlugin::new())
        .add_plugins(ControllingPlugin::new())
        .add_systems(Startup, initial_scene)
        .run();
}
