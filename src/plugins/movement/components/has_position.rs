use bevy::prelude::*;

#[derive(Reflect)]
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
        if value.x == 0. && value.y < 0. {
            Direction::S
        } else if value.x > 0. && value.y < 0. {
            Direction::SE
        } else if value.x > 0. && value.y == 0. {
            Direction::E
        } else if value.x > 0. && value.y > 0. {
            Direction::NE
        } else if value.x == 0. && value.y > 0. {
            Direction::N
        } else if value.x < 0. && value.y > 0. {
            Direction::NW
        } else if value.x < 0. && value.y == 0. {
            Direction::W
        } else {
            Direction::SW
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

impl Direction {
    pub fn should_flip_x(&self) -> bool {
        match &self {
            Direction::NW => true,
            Direction::W => true,
            Direction::SW => true,
            _ => false,
        }
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct HasPosition {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub dir: Direction,
}
