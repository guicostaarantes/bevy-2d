use bevy::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct AbleToControl {
    pub controlling: bool,
    pub key_up: KeyCode,
    pub key_down: KeyCode,
    pub key_left: KeyCode,
    pub key_right: KeyCode,
    pub key_jump: KeyCode,
}

impl Default for AbleToControl {
    fn default() -> Self {
        Self {
            controlling: false,
            key_up: KeyCode::Up,
            key_down: KeyCode::Down,
            key_left: KeyCode::Left,
            key_right: KeyCode::Right,
            key_jump: KeyCode::Space,
        }
    }
}
