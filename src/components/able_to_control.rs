use bevy::prelude::*;

#[derive(Component)]
pub struct AbleToControl {
    pub controlling: bool,
    pub key_up: KeyCode,
    pub key_down: KeyCode,
    pub key_left: KeyCode,
    pub key_right: KeyCode,
    pub key_jump: KeyCode,
}
