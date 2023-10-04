mod movable;

use bevy::prelude::*;
use bevy::{core_pipeline::clear_color::ClearColorConfig, time::Stopwatch};
use movable::{Controllable, Jumpable, Movable};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, move_movable_components)
        .add_systems(Update, move_controllable_characters)
        .add_systems(Update, jump_controllable_characters)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::rgb(0.08, 0.35, 0.12)),
        },
        ..default()
    });

    // Main character
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(150.0, 150.0)),
                ..default()
            },
            texture: asset_server.load("boy.png"),
            ..default()
        },
        Movable {
            walk_speed: 300.0,
            ..default()
        },
        Jumpable {
            jump_duration: 0.5,
            jump_started: None,
        },
        Controllable { controlling: true },
    ));
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
    mut movables: Query<(&mut Movable, &Controllable)>,
    input: Res<Input<KeyCode>>,
) {
    movables.iter_mut().for_each(|(mut movable, controllable)| {
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

            let direction = direction.normalize_or_zero();
            movable.current_speed.x = movable.walk_speed * direction.x;
            movable.current_speed.y = movable.walk_speed * direction.y;
        }
    })
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
