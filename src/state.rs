use amethyst::{
    assets::{
        Handle, Prefab, PrefabLoader, 
        ProgressCounter, RonFormat,
    },
    core::transform::Transform,
    input::{get_key, is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::Camera,
    window::ScreenDimensions,
};

use crate::player::{Ball, Direction};
use crate::prefabs::SpritePrefabData;

use log::info;

#[derive(Default)]
pub struct MyState{
    ball_prefab_progress: Option<ProgressCounter>,

    ball_prefab: Option<Handle<Prefab<SpritePrefabData>>>,
wall_prefab: Option<Handle<Prefab<SpritePrefabData>>>,
}

impl SimpleState for MyState {
    // On start will run when this state is initialized. For more
    // state lifecycle hooks, see:
    // https://book.amethyst.rs/stable/concepts/state.html#life-cycle
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        // Get the screen dimensions so we can initialize the camera and
        // place our sprites correctly later. We'll clone this since we'll
        // pass the world mutably to the following functions.
        let dimensions = world.read_resource::<ScreenDimensions>().clone();

        // Place the camera
        init_camera(world, &dimensions);

        let ball_prefab = self.load_ball_prefab(world);
        self.init_ball(world);
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans{
        if self.wall_prefab == None && self.ball_prefab_progress.as_ref().unwrap().is_complete() {
            self.wall_prefab = Some(data.world.exec(|loader: PrefabLoader<'_, SpritePrefabData>| {
                loader.load("prefabs/wall.ron", RonFormat, ())
            }));
            data.world
                .create_entity()
                .with(self.wall_prefab.as_ref().unwrap().clone())
                .build();
        }
        Trans::None
    }

    fn handle_event(
        &mut self,
        mut _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            // Check if the window should be closed
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }

            // Listen to any key events
            if let Some(event) = get_key(&event) {
                info!("handling key event: {:?}", event);
            }

            // If you're looking for a more sophisticated event handling solution,
            // including key bindings and gamepad support, please have a look at
            // https://book.amethyst.rs/stable/pong-tutorial/pong-tutorial-03.html#capturing-user-input
        }

        // Keep going
        Trans::None
    }
}

fn init_camera(world: &mut World, dimensions: &ScreenDimensions) {
    // Center the camera in the middle of the screen, and let it cover
    // the entire screen
    let mut transform = Transform::default();
    transform.set_translation_xyz(dimensions.width() * 0.5, dimensions.height() * 0.5, 1.);

    world
        .create_entity()
        .with(Camera::standard_2d(dimensions.width(), dimensions.height()))
        .with(transform)
        .build();
}

impl MyState  {
    fn load_ball_prefab(&mut self, world: &mut World) {
        self.ball_prefab_progress = Some(ProgressCounter::new());
        self.ball_prefab = Some(world.exec(|loader: PrefabLoader<'_, SpritePrefabData>| {
            loader.load("prefabs/ball.ron", RonFormat, self.ball_prefab_progress.as_mut().unwrap())
        }));
    }

    fn init_ball(&self, world: &mut World) {
        world.register::<Ball>();
        let mut transform = Transform::default();
        transform.set_translation_xyz(40.0, 40.0, 0.);

        // Create an entity for each sprite and attach the `SpriteRender` as
        // well as the transform. If you want to add behaviour to your sprites,
        // you'll want to add a custom `Component` that will identify them, and a
        // `System` that will iterate over them. See https://book.amethyst.rs/stable/concepts/system.html
        world
            .create_entity()
            .with(self.ball_prefab.as_ref().unwrap().clone())
            .with(transform)
            .with(Ball::new(40.0, 1.0f32, Direction::Right))
            .build();
    }

}

