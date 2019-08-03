use amethyst::{
    ecs::prelude::*,
    assets::{
        AssetStorage, Handle, Loader, Prefab, PrefabData, PrefabLoader, PrefabLoaderSystem,
        ProgressCounter, RonFormat,
    },
    core::transform::Transform,
    input::{get_key, is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    window::ScreenDimensions,
};
use log::info;

pub struct GameState {
    dispatcher: Dispatcher<'static, 'static>,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            dispatcher: DispatcherBuilder::new().build(),
        }
    }
}

impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<GameData>) {
        self.dispatcher.setup(&mut data.world.res);

        let world = data.world;

        let dimensions = world.read_resource::<ScreenDimensions>().clone();

        init_camera(world, &dimensions);
        // let ball_prefab = self.load_ball_prefab(world);
        // self.init_ball(world);
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        self.dispatcher.dispatch(&data.world.res);
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