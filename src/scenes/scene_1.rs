use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

use crate::plugins::animation::components::*;
use crate::plugins::camera::components::*;
use crate::plugins::controlling::components::*;
use crate::plugins::movement::components::*;
use crate::scenes::AppState;

pub fn scene_1(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut change_state: ResMut<NextState<AppState>>,
) {
    // Camera
    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::AutoMax {
        max_width: 640.0,
        max_height: 640.0,
    };
    commands.spawn(camera);

    // Map
    let map = (Name::new("Map"), SpatialBundle { ..default() });
    commands.spawn(map).with_children(|child_builder| {
        child_builder.spawn(SpriteBundle {
            texture: asset_server.load("scene1_z000.png"),
            transform: Transform {
                translation: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                ..default()
            },
            ..default()
        });
        child_builder.spawn(SpriteBundle {
            texture: asset_server.load("scene1_z010.png"),
            transform: Transform {
                translation: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 10.0,
                },
                ..default()
            },
            ..default()
        });
    });

    // Main character
    let player = (
        Name::new("Player"),
        HasPosition {
            x: 0.0,
            y: -60.0,
            z: 0.0,
            ..default()
        },
        AbleToMove {
            walk_speed: 120.0,
            ..default()
        },
        AbleToJump {
            jump_duration: 0.4,
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
        IsFollowedByCamera {
            active: true,
            damping: 15.0,
        },
        SpatialBundle {
            transform: Transform {
                translation: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0,
                },
                ..default()
            },
            ..default()
        },
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

        let walking_texture_atlas = texture_atlases.add(TextureAtlas::from_grid(
            asset_server.load("blue_dummy_walking.png"),
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
                walking_texture_atlas: walking_texture_atlas.clone(),
            },
            SpriteSheetBundle {
                texture_atlas: standing_texture_atlas,
                ..default()
            },
        );
        child_builder.spawn(player_animated);
    });

    change_state.set(AppState::PlayingScene1);
}
