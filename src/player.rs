use amethyst::core::math::{Unit, Vector2};
use amethyst::ecs::prelude::*;

pub struct Ball {
    velocity: f32, // could probably be raised as game gets more difficult
    direction: Unit<Vector2<f32>>,
}
impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn to_vector(self) -> Vector2<f32> {
        match self {
            Up => Vector2::new(0.0, 1.0),
            Down => Vector2::new(0.0, -1.0),
            Left => Vector2::new(-1.0, 0.0),
            Right => Vector2::new(1.0, 0.0),
        }
    }

    fn to_unit_vector(self) -> Unit<Vector2<f32>> {
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