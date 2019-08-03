
use amethyst::{
    assets::{
        Asset, AssetStorage, Handle, Loader, Prefab, PrefabData, PrefabLoader, Progress,
        ProgressCounter, RonFormat,
    },
    core::{
        ecs::{Entity, Read, ReadExpect},
        Named, Transform,
    },
    derive::PrefabData,
    prelude::*,
    renderer::{
        camera::CameraPrefab,
        formats::texture::ImageFormat,
        light::LightPrefab,
        sprite::{SpriteSheet, SpriteSheetFormat},
        transparent::Transparent,
        Texture,
    },
    utils::application_root_dir,
    Error,
};
use crate::prefabs::GamePrefab;
use std::fs::read_dir;
use std::collections::HashMap;
use tiled::{Map, TmxFormat};

fn load_prefab(world: &mut World, path: String, progress: &mut ProgressCounter) -> Option<Handle<Prefab<GamePrefab>>> {
    Some(
        world.exec(|loader: PrefabLoader<'_, GamePrefab>| {
            loader.load(
                path,
                RonFormat,
                &mut *progress,
            )
        })
    )
}

#[derive(Default)]
pub struct PrefabResource {
    ball: Option<Handle<Prefab<GamePrefab>>>,
}

impl<'a> PrefabResource {
    pub fn initialize(world: &mut World, progress: &mut ProgressCounter) {
        let mut prefabs = PrefabResource::default();
        let dir = application_root_dir()
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap()
            + "/resources/prefabs/";

        // lol what is dry
        prefabs.ball = load_prefab(world, format!("{}{}", &dir, "ball.ron"), progress);

        world.add_resource(prefabs);
    }
}

#[derive(Default)]
pub struct MapResource {
    maps: HashMap<String, Handle<Map>>,
}

impl MapResource {
    pub fn initialize(world: &mut World, progress: &mut ProgressCounter) {
        let mut maps = MapResource::default();
        let dir = application_root_dir()
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap()
            + "/resources/maps/";

        type SystemData<'a> = (
            ReadExpect<'a, Loader>,
            Read<'a, AssetStorage<Map>>,
        );
        // Right now, just load a single map. In the future we'll probably want
        // a map manifest file.
        let map_handle = world.exec(|(loader, map_storage): SystemData| {
            loader.load(
                dir + "default.tmx",
                TmxFormat,
                &mut *progress,
                &map_storage,
            )
        });
        maps.maps.insert(String::from("default"), map_handle.clone());
        world.add_resource(maps);
    }
}
