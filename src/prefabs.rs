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

#[derive(Default, Clone, Deserialize, Serialize, PrefabData)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct GamePrefab {
    pub sprite_scene: Option<SpriteScenePrefab>,
}
