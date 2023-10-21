pub mod components;
mod systems;

use bevy::prelude::*;

use self::components::AbleToControl;
use self::systems::control_entities_that_are_able_to_jump_and_able_to_control;
use self::systems::control_entities_that_are_able_to_move_and_able_to_control;

pub struct ControllingPlugin;

impl Plugin for ControllingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            control_entities_that_are_able_to_jump_and_able_to_control,
        )
        .add_systems(
            Update,
            control_entities_that_are_able_to_move_and_able_to_control,
        )
        .register_type::<AbleToControl>();
    }
}

impl ControllingPlugin {
    pub fn new() -> Self {
        Self {}
    }
}
