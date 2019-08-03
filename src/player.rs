use amethyst::core::{
    math::{Unit, Vector2},
    ecs::prelude::*,
};
use specs_derive::Component;

use serde::{Deserialize, Serialize};

#[derive(Component)]
pub struct Ball {
    pub radius: f32,
    pub velocity: f32, // could probably be raised as game gets more difficult
    pub direction: Unit<Vector2<f32>>,
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
    pub fn new(radius: f32, velocity: f32, direction: Direction) -> Self {
        Ball {
            radius,
            velocity,
            direction: direction.to_unit_vector(),
        }
    }
}
