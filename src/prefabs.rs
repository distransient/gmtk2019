use amethyst::{
    assets::{PrefabData, ProgressCounter},
    derive::PrefabData,
    ecs::prelude::*,
    renderer::sprite::prefab::SpriteScenePrefab,
    Error,
};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PrefabData)]
pub struct SpritePrefabData {
    sprite_scene: SpriteScenePrefab,
}
