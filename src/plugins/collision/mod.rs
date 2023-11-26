pub mod components;
mod systems;

use bevy::prelude::*;

use self::components::IsRigidBody;
use self::components::Cuboid;
use self::components::IsVisibleCollisionBox;
use self::systems::collide_rigid_bodies_when_one_is_able_to_move;
use self::systems::draw_collision_boxes;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, collide_rigid_bodies_when_one_is_able_to_move)
            .add_systems(Update, draw_collision_boxes)
            .register_type::<IsRigidBody>()
            .register_type::<Cuboid>()
            .register_type::<IsVisibleCollisionBox>();
    }
}

impl CollisionPlugin {
    pub fn new() -> Self {
        Self {}
    }
}
