use amethyst::{
    assets::{PrefabLoader, ProgressCounter, RonFormat},
    core::{ecs::prelude::*, transform::Transform},
    input::{get_key, is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::Camera,
    window::ScreenDimensions,
};

use crate::environment::{Tile, TilePrefabs};
use crate::player::{Ball, Direction};
use crate::prefabs::SpritePrefabData;

use log::info;

pub struct GameState<'a, 'b> {
    ball_prefab_progress: ProgressCounter,
    fixed_dispatcher: Dispatcher<'a, 'b>,
}

impl<'a, 'b> SimpleState for GameState<'a, 'b> {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let dimensions = world.read_resource::<ScreenDimensions>().clone();

        self.fixed_dispatcher.setup(&mut world.res);
        init_camera(world, &dimensions);
        self.load_ball_prefab(world);
        self.init_ball(world);
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        if self.ball_prefab_progress.is_complete() {
            self.load_all_prefabs(data.world);
            let prefab = {
                let prefabs = data.world.read_resource::<TilePrefabs>();
                prefabs.get_prefab(Tile::Wall).unwrap().clone()
            };
            data.world.create_entity().with(prefab).build();
        }
        Trans::None
    }

    fn fixed_update(&mut self, data: StateData<GameData>) -> SimpleTrans {
        self.fixed_dispatcher.dispatch(&data.world.res);
        Trans::None
    }

    fn handle_event(
        &mut self,
        mut _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }

            if let Some(event) = get_key(&event) {
                info!("handling key event: {:?}", event);
            }
        }
        Trans::None
    }
}

fn init_camera(world: &mut World, dimensions: &ScreenDimensions) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(dimensions.width() * 0.5, dimensions.height() * 0.5, 1.);

    world
        .create_entity()
        .with(Camera::standard_2d(dimensions.width(), dimensions.height()))
        .with(transform)
        .build();
}

impl<'a, 'b> GameState<'a, 'b> {
    pub fn new(
        ball_prefab_progress_counter: ProgressCounter,
        fixed_dispatcher: Dispatcher<'a, 'b>,
    ) -> Self {
        GameState {
            ball_prefab_progress: ball_prefab_progress_counter,
            fixed_dispatcher,
        }
    }

    fn load_ball_prefab(&mut self, world: &mut World) {
        world.add_resource(TilePrefabs::default());
        let prefab = {
            world.exec(|loader: PrefabLoader<'_, SpritePrefabData>| {
                loader.load(
                    "prefabs/ball.ron",
                    RonFormat,
                    &mut self.ball_prefab_progress,
                )
            })
        };
        world
            .write_resource::<TilePrefabs>()
            .insert_prefab(Tile::Ball, prefab);
    }

    fn load_all_prefabs(&mut self, world: &mut World) {
        let prefab = {
            world.exec(|loader: PrefabLoader<'_, SpritePrefabData>| {
                loader.load("prefabs/wall.ron", RonFormat, ())
            })
        };
        let mut tile_prefabs = world.write_resource::<TilePrefabs>();
        tile_prefabs.insert_prefab(Tile::Wall, prefab);
    }

    fn init_ball(&self, world: &mut World) {
        world.register::<Ball>();
        let mut transform = Transform::default();
        transform.set_translation_xyz(40.0, 40.0, 0.);

        let prefab = {
            let prefabs = world.read_resource::<TilePrefabs>();
            prefabs.get_prefab(Tile::Ball).unwrap().clone()
        };
        world
            .create_entity()
            .with(prefab)
            .with(transform)
            .with(Ball::new(40.0, 1.0f32, Direction::Right))
            .build();
    }
}
