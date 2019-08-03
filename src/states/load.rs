use amethyst::{
    prelude::*,
    assets::ProgressCounter,
};

use crate::states::GameState;

pub struct LoadState {
    progress: ProgressCounter,
}

impl LoadState {
    pub fn new() -> Self {
        LoadState {
            progress: ProgressCounter::new()
        }
    }
}

impl SimpleState for LoadState {
    fn on_start(&mut self, mut data: StateData<GameData>) {
        crate::resources::PrefabResource::initialize(data.world, &mut self.progress);
        // Initialize map. In the future, we most likely will want to modify this to load N amount of maps in a directory
        crate::resources::MapResource::initialize(data.world, &mut self.progress);
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        if self.progress.is_complete() {
            self.progress = ProgressCounter::new();
            return Trans::Switch(Box::new(GameState::new()))
        }
        Trans::None
    }
}