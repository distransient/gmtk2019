use amethyst::{
    assets::{PrefabData, ProgressCounter},
    derive::PrefabData,
    ecs::prelude::*,
    prelude::*,
    renderer::sprite::prefab::SpriteScenePrefab,
    Error,
};
use serde::{Deserialize, Serialize};

use crate::player::Ball;

#[derive(Debug, Clone, Deserialize, PrefabData)]
pub struct BallPrefabData {
    sprite_scene: SpriteScenePrefab,
}
