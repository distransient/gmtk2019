use crate::player::Direction;

use serde::{Deserialize, Serialize};

pub enum Tile {
    Nothing,
    Wall,
    Breakable,
    Pusher,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentConfig {
    pub map: String,
    pub start: (f32, f32),
    pub starting_direction: Direction,
    pub player_speed: (f32),
}
