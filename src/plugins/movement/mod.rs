pub mod components;
mod systems;

use bevy::prelude::*;

use self::components::AbleToJump;
use self::components::AbleToMove;
use self::components::Cuboid;
use self::components::Direction;
use self::components::HasPosition;
use self::components::IsRigidBody;
use self::components::IsVisibleCollisionBox;
use self::systems::calculate_position_of_entities_that_are_able_to_jump;
use self::systems::calculate_position_of_entities_that_are_able_to_move;
use self::systems::collide_rigid_bodies_when_one_is_able_to_move;
use self::systems::draw_collision_boxes;
use self::systems::translate_entities_that_have_position;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, calculate_position_of_entities_that_are_able_to_jump)
            .add_systems(Update, calculate_position_of_entities_that_are_able_to_move)
            .add_systems(
                Update,
                collide_rigid_bodies_when_one_is_able_to_move
                    .after(translate_entities_that_have_position),
            )
            .add_systems(Update, translate_entities_that_have_position)
            .add_systems(Update, draw_collision_boxes)
            .register_type::<AbleToJump>()
            .register_type::<AbleToMove>()
            .register_type::<Cuboid>()
            .register_type::<Direction>()
            .register_type::<HasPosition>()
            .register_type::<IsRigidBody>()
            .register_type::<IsVisibleCollisionBox>();
    }
}

impl MovementPlugin {
    pub fn new() -> Self {
        Self {}
    }
}
