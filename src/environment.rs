use crate::{player::Direction, prefabs::SpritePrefabData};
use amethyst::assets::{Handle, Prefab};
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Tile {
    Nothing,
    Wall,
    Breakable,
    Pusher,
    Ball,
}

#[derive(Default)]
pub struct TilePrefabs {
    prefabs: HashMap<Tile, Handle<Prefab<SpritePrefabData>>>,
}

impl TilePrefabs {
    pub fn insert_prefab(&mut self, tile: Tile, prefab: Handle<Prefab<SpritePrefabData>>) {
        self.prefabs.insert(tile, prefab);
    }

    pub fn get_prefab(&self, tile: Tile) -> Option<&Handle<Prefab<SpritePrefabData>>> {
        self.prefabs.get(&tile)
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentConfig {
    pub map: String,
    pub start: (f32, f32),
    pub starting_direction: Direction,
    pub player_speed: (f32),
}
