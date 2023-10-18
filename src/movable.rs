use bevy::{prelude::*, time::Stopwatch};

pub enum Direction {
    S,
    SE,
    E,
    NE,
    N,
    NW,
    W,
    SW,
}

impl Default for Direction {
    fn default() -> Self {
        Self::S
    }
}

impl From<Vec2> for Direction {
    fn from(value: Vec2) -> Self {
        if value.x == 0.0 && value.y < 0.0 {
            crate::movable::Direction::S
        } else if value.x > 0.0 && value.y < 0.0 {
            crate::movable::Direction::SE
        } else if value.x > 0.0 && value.y == 0.0 {
            crate::movable::Direction::E
        } else if value.x > 0.0 && value.y > 0.0 {
            crate::movable::Direction::NE
        } else if value.x == 0.0 && value.y > 0.0 {
            crate::movable::Direction::N
        } else if value.x < 0.0 && value.y > 0.0 {
            crate::movable::Direction::NW
        } else if value.x < 0.0 && value.y == 0.0 {
            crate::movable::Direction::W
        } else {
            crate::movable::Direction::SW
        }
    }
}

impl Direction {
    pub fn get_sprite_index(&self) -> usize {
        match &self {
            Direction::S => 0,
            Direction::SE => 1,
            Direction::E => 2,
            Direction::NE => 3,
            Direction::N => 4,
            Direction::NW => 3,
            Direction::W => 2,
            Direction::SW => 1,
        }
    }
}

#[derive(Component, Default)]
pub struct Movable {
    pub walk_speed: f32,
    pub current_speed: Vec2,
    pub current_position: Vec2,
    pub direction: Direction,
    pub standing_texture_atlas: Handle<TextureAtlas>,
    pub walking_texture_atlas: Handle<TextureAtlas>,
}

#[derive(Component, Default)]
pub struct Animated {
    pub timer_to_animate: Timer,
    pub current_frame: usize,
    pub number_of_frames: usize,
}

#[derive(Component, Default)]
pub struct Jumpable {
    pub jump_duration: f32,
    pub jump_started: Option<Stopwatch>,
}

#[derive(Component, Default)]
pub struct Controllable {
    pub controlling: bool,
    pub timer_to_control: Timer,
}
