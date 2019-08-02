use amethyst::{
    Error,
    prelude::*,
    ecs::prelude::*,
    renderer::sprite::prefab::SpriteScenePrefab,
    assets::{ProgressCounter, PrefabData},
    derive::PrefabData,
};
use serde::{Deserialize, Serialize};

use crate::player::Ball;

#[derive(Debug, Clone, Deserialize, PrefabData)]
pub struct BallPrefabData {
    sprite_scene: SpriteScenePrefab,
}