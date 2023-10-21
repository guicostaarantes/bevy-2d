use bevy::{prelude::*, time::Stopwatch};

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct AbleToJump {
    pub jump_duration: f32,
    pub jump_started: Option<Stopwatch>,
}
