use crate::player::Direction;

use serde::{Serialize, Deserialize};

pub enum Tile {
    Nothing, 
    Wall,
    Breakable {broken: bool},
    Pusher,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentConfig {
    pub map: String,
    pub start: (f32, f32),
    pub starting_direction: Direction,
    pub player_speed: (f32),
}