use amethyst::{
    Error,
    prelude::*,
    ecs::prelude::*,
    renderer::sprite::prefab::SpriteScenePrefab,
    assets::{ProgressCounter, PrefabData},
    derive::PrefabData,
};
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Deserialize, PrefabData)]
pub struct BallPrefabData {
    sprite_scene: SpriteScenePrefab,
}