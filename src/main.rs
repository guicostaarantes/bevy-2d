mod components;
mod systems;

use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::{core_pipeline::clear_color::ClearColorConfig, time::Stopwatch};
use components::able_to_animate::AbleToAnimate;
use components::able_to_control::AbleToControl;
use components::able_to_jump::AbleToJump;
use components::able_to_move::AbleToMove;
use components::has_position::HasPosition;

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
        .add_systems(Startup, setup_scene)
        .add_systems(Update, calculate_position_of_entities_that_are_able_to_move)
        .add_systems(Update, calculate_position_of_entities_that_are_able_to_jump)
        .add_systems(Update, translate_entities_that_have_position)
        .add_systems(
            Update,
            control_entities_that_are_able_to_move_and_able_to_control,
        )
        .add_systems(
            Update,
            control_entities_that_are_able_to_jump_and_able_to_control,
        )
        .add_systems(Update, animate_entities_that_are_able_to_animate)
        .run();
}

fn setup_scene(
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
                walking_texture_atlas: Some(texture_atlases.add(TextureAtlas::from_grid(
                    asset_server.load("blue_dummy_walking.png"),
                    Vec2::new(32.0, 48.0),
                    40,
                    1,
                    None,
                    None,
                ))),
            },
            SpriteSheetBundle {
                texture_atlas: standing_texture_atlas,
                ..default()
            },
        );

        child_builder.spawn(player_animated);
    });
}

fn calculate_position_of_entities_that_are_able_to_move(
    mut movables: Query<(&mut HasPosition, &AbleToMove)>,
    time: Res<Time>,
) {
    movables.iter_mut().for_each(|(mut pos, movable)| {
        pos.x += movable.current_speed.x * time.delta_seconds();
        pos.y += movable.current_speed.y * time.delta_seconds();
    });
}

fn calculate_position_of_entities_that_are_able_to_jump(
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

fn translate_entities_that_have_position(mut positionables: Query<(&mut Transform, &HasPosition)>) {
    positionables.iter_mut().for_each(|(mut tf, pos)| {
        tf.translation.x = pos.x;
        tf.translation.y = pos.y + pos.z;
    })
}

fn control_entities_that_are_able_to_move_and_able_to_control(
    mut movables: Query<(&mut HasPosition, &mut AbleToMove, &AbleToControl)>,
    input: Res<Input<KeyCode>>,
) {
    movables
        .iter_mut()
        .for_each(|(mut pos, mut movable, controllable)| {
            if controllable.controlling {
                let mut direction = Vec2::new(0.0, 0.0);

                if input.pressed(controllable.key_up) {
                    direction.y += 1.0;
                }
                if input.pressed(controllable.key_left) {
                    direction.x -= 1.0;
                }
                if input.pressed(controllable.key_down) {
                    direction.y -= 1.0;
                }
                if input.pressed(controllable.key_right) {
                    direction.x += 1.0;
                }

                let direction_vector = direction.normalize_or_zero();
                movable.current_speed.x = movable.walk_speed * direction_vector.x;
                movable.current_speed.y = movable.walk_speed * direction_vector.y;

                if direction_vector.x != 0.0 || direction_vector.y != 0.0 {
                    pos.dir = direction_vector.into();
                }
            }
        })
}

fn control_entities_that_are_able_to_jump_and_able_to_control(
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

fn animate_entities_that_are_able_to_animate(
    movables: Query<(&HasPosition, Option<&AbleToMove>)>,
    mut sprites: Query<(
        &mut AbleToAnimate,
        &mut Handle<TextureAtlas>,
        &mut TextureAtlasSprite,
    )>,
    time: Res<Time>,
) {
    movables.iter().for_each(|(position, movable)| {
        sprites
            .iter_mut()
            .for_each(|(mut animated, mut atlas_handle, mut sprite)| {
                animated.timer_to_animate.tick(time.delta());
                if !animated.timer_to_animate.just_finished() {
                    return;
                }

                if animated.current_frame + 1 == animated.number_of_frames {
                    animated.current_frame = 0;
                } else {
                    animated.current_frame += 1;
                }
                sprite.index = animated.number_of_frames * position.dir.get_sprite_index()
                    + animated.current_frame;
                sprite.flip_x = position.dir.should_flip_x();

                if let Some(walking_atlas) = &animated.walking_texture_atlas {
                    if let Some(mov) = movable {
                        if *atlas_handle == animated.texture_atlas
                            && mov.current_speed.length() > 0.0
                        {
                            *atlas_handle = walking_atlas.clone()
                        } else if *atlas_handle == *walking_atlas
                            && mov.current_speed.length() == 0.0
                        {
                            *atlas_handle = animated.texture_atlas.clone()
                        }
                    }
                }
            });
    });
}
