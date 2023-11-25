use bevy::prelude::*;

use crate::plugins::collision::components::*;
use crate::plugins::movement::components::AbleToMove;
use crate::plugins::movement::components::HasPosition;

pub fn collide_entities_when_one_is_able_to_move(
    // mut commands: Commands,
    mut movable_colliders: Query<(&Name, &IsRigidBody, &mut HasPosition), (With<AbleToMove>, Changed<HasPosition>)>,
    immovable_colliders: Query<(&Name, &IsRigidBody, &HasPosition), Without<AbleToMove>>,
) {
    immovable_colliders.iter().for_each(|(immo_entity, immo_rigid_body, immo_position)| {
        movable_colliders.iter_mut().for_each(|(mov_entity, mov_rigid_body, mov_position)| {
            immo_rigid_body.cuboids.iter().for_each(|immo_cuboid| {
                mov_rigid_body.cuboids.iter().for_each(|mov_cuboid| {
                    // x axis
                    let immo_cuboid_min = immo_position.x + immo_cuboid.displacement.x - (immo_cuboid.size.x / 2.);
                    let immo_cuboid_max = immo_position.x + immo_cuboid.displacement.x + (immo_cuboid.size.x / 2.);
                    let mov_cuboid_min = mov_position.x + mov_cuboid.displacement.x - (mov_cuboid.size.x / 2.);
                    let mov_cuboid_max = mov_position.x + mov_cuboid.displacement.x + (mov_cuboid.size.x / 2.);
                    if immo_cuboid_min > mov_cuboid_max || mov_cuboid_min > immo_cuboid_max { return; }

                    // y axis
                    let immo_cuboid_min = immo_position.y + immo_cuboid.displacement.y - (immo_cuboid.size.y / 2.);
                    let immo_cuboid_max = immo_position.y + immo_cuboid.displacement.y + (immo_cuboid.size.y / 2.);
                    let mov_cuboid_min = mov_position.y + mov_cuboid.displacement.y - (mov_cuboid.size.y / 2.);
                    let mov_cuboid_max = mov_position.y + mov_cuboid.displacement.y + (mov_cuboid.size.y / 2.);
                    if immo_cuboid_min > mov_cuboid_max || mov_cuboid_min > immo_cuboid_max { return; }
                    
                    // z axis
                    let immo_cuboid_min = immo_position.z + immo_cuboid.displacement.z - (immo_cuboid.size.z / 2.);
                    let immo_cuboid_max = immo_position.z + immo_cuboid.displacement.z + (immo_cuboid.size.z / 2.);
                    let mov_cuboid_min = mov_position.z + mov_cuboid.displacement.z - (mov_cuboid.size.z / 2.);
                    let mov_cuboid_max = mov_position.z + mov_cuboid.displacement.z + (mov_cuboid.size.z / 2.);
                    if immo_cuboid_min > mov_cuboid_max || mov_cuboid_min > immo_cuboid_max { return; }

                    println!("{} and {} are colliding", immo_entity, mov_entity);
                });
            });
        });
    });
}
