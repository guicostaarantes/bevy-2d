use bevy::prelude::*;

use crate::plugins::collision::components::*;
use crate::plugins::movement::components::HasPosition;

pub fn draw_collision_boxes(
    mut commands: Commands,
    colliders: Query<
        (Entity, Option<&Children>, &IsRigidBody),
        (
            With<HasPosition>,
            Or<(Changed<IsRigidBody>, Changed<HasPosition>)>,
        ),
    >,
    collision_boxes: Query<Entity, With<IsVisibleCollisionBox>>,
) {
    colliders.iter().for_each(|(entity, children, collide)| {
        if let Some(children) = children {
            children
                .iter()
                .filter_map(|child| collision_boxes.get(*child).ok())
                .for_each(|ent| commands.entity(ent).despawn_recursive())
        };

        if collide.show_collision_boxes {
            commands.entity(entity).with_children(|child_builder| {
                collide.cuboids.iter().for_each(|cuboid| {
                    child_builder.spawn((
                        IsVisibleCollisionBox,
                        SpriteBundle {
                            sprite: Sprite {
                                color: Color::Rgba {
                                    red: 0.,
                                    green: 1.,
                                    blue: 0.,
                                    alpha: 0.3,
                                },
                                custom_size: Some(Vec2::new(cuboid.size.x, cuboid.size.z)),
                                ..default()
                            },
                            transform: Transform {
                                translation: Vec3 {
                                    x: cuboid.displacement.x,
                                    y: cuboid.displacement.y + cuboid.displacement.z
                                        - cuboid.size.y / 2.,
                                    z: 10.,
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
                                    red: 0.,
                                    green: 1.,
                                    blue: 1.,
                                    alpha: 0.3,
                                },
                                custom_size: Some(Vec2::new(cuboid.size.x, cuboid.size.y)),
                                ..default()
                            },
                            transform: Transform {
                                translation: Vec3 {
                                    x: cuboid.displacement.x,
                                    y: cuboid.displacement.y
                                        + cuboid.displacement.z
                                        + cuboid.size.z / 2.,
                                    z: 10.,
                                },
                                ..default()
                            },
                            ..default()
                        },
                    ));
                });
            });
        }
    });
}
