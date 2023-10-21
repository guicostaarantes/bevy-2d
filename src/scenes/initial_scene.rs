use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

use crate::plugins::animation::components::*;
use crate::plugins::controlling::components::*;
use crate::plugins::movement::components::*;

pub fn initial_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Camera
    let mut camera = Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::rgb(0.08, 0.35, 0.12)),
        },
        ..default()
    };
    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 360.0,
        min_height: 360.0,
    };
    commands.spawn(camera);

    // Main character
    let player = (
        HasPosition {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            ..default()
        },
        AbleToMove {
            walk_speed: 100.0,
            ..default()
        },
        AbleToJump {
            jump_duration: 0.35,
            jump_started: None,
        },
        AbleToControl {
            controlling: true,
            key_up: KeyCode::W,
            key_down: KeyCode::S,
            key_left: KeyCode::A,
            key_right: KeyCode::D,
            key_jump: KeyCode::Space,
        },
        SpatialBundle::default(),
    );
    commands.spawn(player).with_children(|child_builder| {
        let standing_texture_atlas = texture_atlases.add(TextureAtlas::from_grid(
            asset_server.load("blue_dummy_standing.png"),
            Vec2::new(32.0, 48.0),
            40,
            1,
            None,
            None,
        ));

        let player_animated = (
            AbleToAnimate {
                timer_to_animate: Timer::from_seconds(0.07, TimerMode::Repeating),
                current_frame: 0,
                number_of_frames: 8,
                texture_atlas: standing_texture_atlas.clone(),
                walking_texture_atlas: texture_atlases.add(TextureAtlas::from_grid(
                    asset_server.load("blue_dummy_walking.png"),
                    Vec2::new(32.0, 48.0),
                    40,
                    1,
                    None,
                    None,
                )),
            },
            SpriteSheetBundle {
                texture_atlas: standing_texture_atlas,
                ..default()
            },
        );

        child_builder.spawn(player_animated);
    });
}
