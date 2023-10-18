use bevy::prelude::*;

#[derive(Component, Default)]
pub struct AbleToAnimate {
    pub timer_to_animate: Timer,
    pub current_frame: usize,
    pub number_of_frames: usize,
    pub texture_atlas: Handle<TextureAtlas>,
    pub walking_texture_atlas: Option<Handle<TextureAtlas>>,
}
