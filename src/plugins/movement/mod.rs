pub mod components;
mod systems;

use bevy::prelude::*;

use self::components::AbleToJump;
use self::components::AbleToMove;
use self::components::Direction;
use self::components::HasPosition;
use self::systems::calculate_position_of_entities_that_are_able_to_jump;
use self::systems::calculate_position_of_entities_that_are_able_to_move;
use self::systems::translate_entities_that_have_position;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, calculate_position_of_entities_that_are_able_to_jump)
            .add_systems(Update, calculate_position_of_entities_that_are_able_to_move)
            .add_systems(Update, translate_entities_that_have_position)
            .register_type::<AbleToJump>()
            .register_type::<AbleToMove>()
            .register_type::<HasPosition>()
            .register_type::<Direction>();
    }
}

impl MovementPlugin {
    pub fn new() -> Self {
        Self {}
    }
}
