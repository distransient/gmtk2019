use crate::{player::Direction, prefabs::SpritePrefabData};
use amethyst::assets::{Handle, Prefab};
use std::collections::HashMap;

use amethyst::core::{
    math::{Unit, Vector2},
    components::Transform,
    ecs::prelude::*,
};
use serde::{Deserialize, Serialize};
use specs_static::Storage as StaticStorage;
use specs_derive::Component;

pub type TileMap = StaticStorage<Tile, <Tile as Component>::Storage, (u32, u32)>;

/// Resource to track offset+scale of level's tiles from Transform 0,0 to index 0,0
#[derive(Default, Clone, Debug)]
pub struct LevelOffset(pub Transform);

pub fn attempt_bounce(from_transform: &Transform, on_map: &TileMap, with_offset: &LevelOffset) -> Option<Unit<Vector2<f32>>> {
    let transformed = from_transform.matrix() * with_offset.0.matrix();
    let index = (transformed.column(3).x as u32, transformed.column(3).y as u32);

    if on_map.get((index.0, index.1 + 1)).map_or(false, |tile| tile == Tile::Pusher) {
        Some((Direction::Up).to_unit_vector())
    } else if on_map.get((index.0, index.1 - 1)).map_or(false, |tile| tile == Tile::Pusher) {
        Some((Direction::Down).to_unit_vector())
    } else if on_map.get((index.0 - 1, index.1)).map_or(false, |tile| tile == Tile::Pusher) {
        Some((Direction::Left).to_unit_vector())
    } else if on_map.get((index.0 + 1, index.1)).map_or(false, |tile| tile == Tile::Pusher) {
        Some((Direction::Right).to_unit_vector())
    } else {
        None
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Component)]
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
