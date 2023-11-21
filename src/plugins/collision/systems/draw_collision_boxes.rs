use bevy::prelude::*;

use crate::plugins::collision::components::*;
use crate::plugins::movement::components::HasPosition;

pub fn draw_collision_boxes(
    mut commands: Commands,
    mut colliders: Query<(Entity, &Children, &AbleToCollide, Option<&HasPosition>)>,
    mut collision_boxes: Query<(Entity, &mut Transform), With<IsVisibleCollisionBox>>,
) {
    colliders
        .iter_mut()
        .for_each(|(entity, children, collide, _pos)| {
            let has_child_collision_box = children
                .iter()
                .any(|child| collision_boxes.get(*child).is_ok());

            if has_child_collision_box {
                children.iter().for_each(|child| {
                    if let Ok((ent, _transform)) = collision_boxes.get_mut(*child) {
                        if !collide.show_collision_boxes {
                            commands.entity(ent).despawn_recursive();
                        } else {
                            // Move collision box according to HasPosition
                        }
                    }
                });
            } else {
                if collide.show_collision_boxes {
                    // Create visible collision box
                    commands.entity(entity).with_children(|child_builder| {
                        collide.cuboids.iter().for_each(|cuboid| {
                            child_builder.spawn((
                                IsVisibleCollisionBox,
                                SpriteBundle {
                                    sprite: Sprite {
                                        color: Color::Rgba {
                                            red: 1.,
                                            green: 0.,
                                            blue: 0.,
                                            alpha: 0.5,
                                        },
                                        custom_size: Some(Vec2::new(cuboid.size.x, cuboid.size.z)),
                                        ..default()
                                    },
                                    transform: Transform {
                                        translation: Vec3 {
                                            x: cuboid.displacement.x,
                                            y: (cuboid.displacement.y + cuboid.displacement.z),
                                            z: 990.,
                                        },
                                        ..default()
                                    },
                                    ..default()
                                },
                            ));
                            child_builder.spawn((
                                IsVisibleCollisionBox,
                                SpriteBundle {
                                    sprite: Sprite {
                                        color: Color::Rgba {
                                            red: 1.,
                                            green: 0.,
                                            blue: 1.,
                                            alpha: 0.5,
                                        },
                                        custom_size: Some(Vec2::new(cuboid.size.x, cuboid.size.y)),
                                        ..default()
                                    },
                                    transform: Transform {
                                        translation: Vec3 {
                                            x: cuboid.displacement.x,
                                            y: (cuboid.displacement.y + cuboid.displacement.z)
                                                + (cuboid.size.y + cuboid.size.z) / 2.,
                                            z: 990.,
                                        },
                                        ..default()
                                    },
                                    ..default()
                                },
                            ));
                        });
                    });
                }
            }
        });
}
