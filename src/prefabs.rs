use amethyst::assets::{Handle, Prefab};
use amethyst::{
    assets::{PrefabData, ProgressCounter},
    derive::PrefabData,
    ecs::prelude::*,
    renderer::sprite::prefab::SpriteScenePrefab,
    Error,
};
use serde::Deserialize;
use std::collections::HashMap;

use crate::environment::Tile;

#[derive(Debug, Clone, Deserialize, PrefabData)]
pub struct SpritePrefabData {
    sprite_scene: SpriteScenePrefab,
}

#[derive(Default)]
pub struct TilePrefabs {
    prefabs: HashMap<Tile, Handle<Prefab<SpritePrefabData>>>,
}

impl TilePrefabs {
    pub fn count(&self) -> usize {
        self.prefabs.len()
    }

    pub fn insert_prefab(&mut self, tile: Tile, prefab: Handle<Prefab<SpritePrefabData>>) {
        self.prefabs.insert(tile, prefab);
    }

    pub fn get_prefab(&self, tile: Tile) -> Option<&Handle<Prefab<SpritePrefabData>>> {
        self.prefabs.get(&tile)
    }
}
