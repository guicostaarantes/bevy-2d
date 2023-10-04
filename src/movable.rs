use bevy::{prelude::*, time::Stopwatch};

#[derive(Component, Default)]
pub struct Movable {
    pub walk_speed: f32,
    pub current_speed: Vec2,
    pub current_position: Vec2,
}

#[derive(Component, Default)]
pub struct Jumpable {
    pub jump_duration: f32,
    pub jump_started: Option<Stopwatch>,
}

#[derive(Component)]
pub struct Controllable {
    pub controlling: bool,
}
