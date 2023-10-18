mod movable;

use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::{core_pipeline::clear_color::ClearColorConfig, time::Stopwatch};
use movable::{Animated, Controllable, Jumpable, Movable};

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
        .add_systems(Startup, setup)
        .add_systems(Update, move_movable_components)
        .add_systems(Update, move_controllable_characters)
        .add_systems(Update, animate_movable_sprite_sheet)
        .add_systems(Update, jump_controllable_characters)
        .run();
}

fn setup(
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
    let standing_texture_atlas = texture_atlases.add(TextureAtlas::from_grid(
        asset_server.load("blue_dummy_standing.png"),
        Vec2::new(32.0, 48.0),
        40,
        1,
        None,
        None,
    ));
    let player = (
        Movable {
            walk_speed: 100.0,
            standing_texture_atlas: standing_texture_atlas.clone(),
            walking_texture_atlas: texture_atlases.add(TextureAtlas::from_grid(
                asset_server.load("blue_dummy_walking.png"),
                Vec2::new(32.0, 48.0),
                40,
                1,
                None,
                None,
            )),
            ..default()
        },
        Jumpable {
            jump_duration: 0.35,
            jump_started: None,
        },
        Controllable {
            controlling: true,
            timer_to_control: Timer::from_seconds(0.1, TimerMode::Repeating),
        },
        Animated {
            timer_to_animate: Timer::from_seconds(0.07, TimerMode::Repeating),
            current_frame: 0,
            number_of_frames: 8,
        },
        SpatialBundle::default(),
    );
    commands.spawn(player).with_children(|child_builder| {
        child_builder.spawn(SpriteSheetBundle {
            texture_atlas: standing_texture_atlas,
            ..default()
        });
    });
}

fn move_movable_components(
    mut movables: Query<(&mut Transform, Option<&mut Movable>, Option<&mut Jumpable>)>,
    time: Res<Time>,
) {
    for (mut transform, mut movable, mut jumpable) in &mut movables {
        movable.as_mut().map(|mv| {
            mv.current_position.x += mv.current_speed.x * time.delta_seconds();
            mv.current_position.y += mv.current_speed.y * time.delta_seconds();
            transform.translation.x = mv.current_position.x;
            transform.translation.y = mv.current_position.y;
        });

        jumpable.as_mut().map(|jp| match &mut jp.jump_started {
            Some(sw) => {
                sw.tick(time.delta());
                let elapsed = sw.elapsed_secs();
                let jump_current_height = 1000.0 * elapsed * (jp.jump_duration - elapsed);
                if jump_current_height < 0.0 {
                    jp.jump_started = None;
                } else {
                    transform.translation.y += jump_current_height;
                }
            }
            None => {}
        });
    }
}

fn move_controllable_characters(
    mut movables: Query<(&mut Movable, &mut Controllable)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    movables
        .iter_mut()
        .for_each(|(mut movable, mut controllable)| {
            controllable.timer_to_control.tick(time.delta());
            if !controllable.timer_to_control.just_finished() {
                return;
            }

            if controllable.controlling {
                let mut direction = Vec2::new(0.0, 0.0);

                if input.pressed(KeyCode::W) {
                    direction.y += 1.0;
                }
                if input.pressed(KeyCode::A) {
                    direction.x -= 1.0;
                }
                if input.pressed(KeyCode::S) {
                    direction.y -= 1.0;
                }
                if input.pressed(KeyCode::D) {
                    direction.x += 1.0;
                }

                let direction_vector = direction.normalize_or_zero();
                movable.current_speed.x = movable.walk_speed * direction_vector.x;
                movable.current_speed.y = movable.walk_speed * direction_vector.y;

                if direction_vector.x != 0.0 || direction_vector.y != 0.0 {
                    movable.direction = direction_vector.into();
                }
            }
        })
}

fn animate_movable_sprite_sheet(
    mut movables: Query<(&mut Animated, &Movable)>,
    mut sprites: Query<(&mut Handle<TextureAtlas>, &mut TextureAtlasSprite)>,
    time: Res<Time>,
) {
    movables.iter_mut().for_each(|(mut animated, movable)| {
        animated.timer_to_animate.tick(time.delta());
        if !animated.timer_to_animate.just_finished() {
            return;
        }

        if animated.current_frame + 1 == animated.number_of_frames {
            animated.current_frame = 0;
        } else {
            animated.current_frame += 1;
        }
        let direction_index = movable.direction.get_sprite_index();

        sprites
            .iter_mut()
            .for_each(|(mut atlas_handle, mut sprite)| {
                if *atlas_handle == movable.standing_texture_atlas
                    && movable.current_speed.length() > 0.0
                {
                    *atlas_handle = movable.walking_texture_atlas.clone()
                } else if *atlas_handle == movable.walking_texture_atlas
                    && movable.current_speed.length() == 0.0
                {
                    *atlas_handle = movable.standing_texture_atlas.clone()
                }
                sprite.index = animated.number_of_frames * direction_index + animated.current_frame;
                sprite.flip_x = match movable.direction {
                    movable::Direction::NW => true,
                    movable::Direction::W => true,
                    movable::Direction::SW => true,
                    _ => false,
                }
            });
    });
}

fn jump_controllable_characters(
    mut jumpables: Query<(&mut Jumpable, &Controllable)>,
    input: Res<Input<KeyCode>>,
) {
    jumpables
        .iter_mut()
        .for_each(|(mut jumpable, controllable)| {
            if controllable.controlling
                && input.just_pressed(KeyCode::Space)
                && jumpable.jump_started == None
            {
                jumpable.jump_started = Some(Stopwatch::new());
            }
        })
}
