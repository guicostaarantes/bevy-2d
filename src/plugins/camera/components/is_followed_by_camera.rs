use bevy::prelude::*;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct IsFollowedByCamera {
    pub active: bool,
    pub damping: f32,
}
