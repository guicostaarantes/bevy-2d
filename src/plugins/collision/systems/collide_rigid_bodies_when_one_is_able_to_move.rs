use bevy::prelude::*;

use crate::plugins::collision::components::*;
use crate::plugins::movement::components::AbleToMove;
use crate::plugins::movement::components::HasPosition;

const BOUNCE_MULTIPLIER: f32 = 1.5;

pub fn collide_rigid_bodies_when_one_is_able_to_move(
    // mut commands: Commands,
    mut movable_colliders: Query<(&IsRigidBody, &mut HasPosition, &mut AbleToMove), Changed<HasPosition>>,
    immovable_colliders: Query<(&IsRigidBody, &HasPosition), Without<AbleToMove>>,
    time: Res<Time>,
) {
    immovable_colliders.iter().for_each(|(immo_rigid_body, immo_position)| {
        movable_colliders.iter_mut().for_each(|(mov_rigid_body, mut position, mut movable)| {
            immo_rigid_body.cuboids.iter().for_each(|immo_cuboid| {
                mov_rigid_body.cuboids.iter().for_each(|mov_cuboid| {
                    // x axis
                    let immo_cuboid_max = immo_position.x + immo_cuboid.displacement.x + (immo_cuboid.size.x / 2.);
                    let mov_cuboid_min = position.x + mov_cuboid.displacement.x - (mov_cuboid.size.x / 2.);
                    let diff_west = immo_cuboid_max - mov_cuboid_min;
                    if diff_west < 0. { return; }
                    let immo_cuboid_min = immo_position.x + immo_cuboid.displacement.x - (immo_cuboid.size.x / 2.);
                    let mov_cuboid_max = position.x + mov_cuboid.displacement.x + (mov_cuboid.size.x / 2.);
                    let diff_east = mov_cuboid_max - immo_cuboid_min;
                    if diff_east < 0. { return; }
                    let hit_from_west = diff_west <= -movable.current_speed.x * time.delta_seconds();
                    let hit_from_east = diff_east <= movable.current_speed.x * time.delta_seconds();

                    // y axis
                    let immo_cuboid_max = immo_position.y + immo_cuboid.displacement.y + (immo_cuboid.size.y / 2.);
                    let mov_cuboid_min = position.y + mov_cuboid.displacement.y - (mov_cuboid.size.y / 2.);
                    let diff_south = immo_cuboid_max - mov_cuboid_min;
                    if diff_south < 0. { return; }
                    let immo_cuboid_min = immo_position.y + immo_cuboid.displacement.y - (immo_cuboid.size.y / 2.);
                    let mov_cuboid_max = position.y + mov_cuboid.displacement.y + (mov_cuboid.size.y / 2.);
                    let diff_north = mov_cuboid_max - immo_cuboid_min;
                    if diff_north < 0. { return; }
                    let hit_from_south = diff_south <= -movable.current_speed.y * time.delta_seconds();
                    let hit_from_north = diff_north <= movable.current_speed.y * time.delta_seconds();
                    
                    // z axis
                    let immo_cuboid_max = immo_position.z + immo_cuboid.displacement.z + (immo_cuboid.size.z / 2.);
                    let mov_cuboid_min = position.z + mov_cuboid.displacement.z - (mov_cuboid.size.z / 2.);
                    let diff_below = immo_cuboid_max - mov_cuboid_min;
                    if diff_below < 0. { return; }
                    let immo_cuboid_min = immo_position.z + immo_cuboid.displacement.z - (immo_cuboid.size.z / 2.);
                    let mov_cuboid_max = position.z + mov_cuboid.displacement.z + (mov_cuboid.size.z / 2.);
                    let diff_above = mov_cuboid_max - immo_cuboid_min;
                    if diff_above < 0. { return; }

                    if hit_from_west {
                        movable.current_speed.x = 0.;
                        position.x += BOUNCE_MULTIPLIER * diff_west;
                    } else if hit_from_east {
                        movable.current_speed.x = 0.;
                        position.x -= BOUNCE_MULTIPLIER * diff_east;
                    }
                    if hit_from_south {
                        movable.current_speed.y = 0.;
                        position.y += BOUNCE_MULTIPLIER * diff_south;
                    } else if hit_from_north {
                        movable.current_speed.y = 0.;
                        position.y -= BOUNCE_MULTIPLIER * diff_north;
                    }
                });
            });
        });
    });
}
