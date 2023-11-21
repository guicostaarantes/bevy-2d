pub mod components;
mod systems;

use bevy::prelude::*;

use self::components::AbleToCollide;
use self::components::Cuboid;
use self::components::IsVisibleCollisionBox;
use self::systems::draw_collision_boxes;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, draw_collision_boxes)
            .register_type::<AbleToCollide>()
            .register_type::<Cuboid>()
            .register_type::<IsVisibleCollisionBox>();
    }
}

impl CollisionPlugin {
    pub fn new() -> Self {
        Self {}
    }
}
