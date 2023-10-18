use bevy::prelude::*;

#[derive(Component, Default)]
pub struct AbleToMove {
    pub walk_speed: f32,
    pub current_speed: Vec2,
}
