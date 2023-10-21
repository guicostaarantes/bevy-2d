pub mod components;
mod systems;

use bevy::prelude::*;

use self::components::AbleToAnimate;
use self::systems::animate_entities_that_are_able_to_animate;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate_entities_that_are_able_to_animate)
            .register_type::<AbleToAnimate>();
    }
}

impl AnimationPlugin {
    pub fn new() -> Self {
        Self {}
    }
}
