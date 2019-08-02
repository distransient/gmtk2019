use amethyst::{
    core::math::{Unit, Vector2},
    ecs::{Component, DenseVecStorage},
};

use serde::{Deserialize, Serialize};

pub struct Ball {
    velocity: f32, // could probably be raised as game gets more difficult
    direction: Unit<Vector2<f32>>,
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Default for Direction {
    fn default() -> Self {
        Direction::Up
    }
}

impl Direction {
    pub fn to_vector(self) -> Vector2<f32> {
        match self {
            Direction::Up => Vector2::new(0.0, 1.0),
            Direction::Down => Vector2::new(0.0, -1.0),
            Direction::Left => Vector2::new(-1.0, 0.0),
            Direction::Right => Vector2::new(1.0, 0.0),
        }
    }

    pub fn to_unit_vector(self) -> Unit<Vector2<f32>> {
        Unit::new_normalize(self.to_vector())
    }
}

impl Ball {
    pub fn new(velocity: f32, direction: Direction) -> Self {
        Ball {
            velocity, 
            direction: direction.to_unit_vector(),
        }
    }
}