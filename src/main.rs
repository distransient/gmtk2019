extern crate specs_derive;

use amethyst::{
    assets::PrefabLoaderSystem,
    core::transform::TransformBundle,
    input::StringBindings,
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
};

mod environment;
mod player;
mod prefabs;
mod state;
mod systems;

use crate::prefabs::SpritePrefabData;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let resources = app_root.join("resources");
    let display_config = resources.join("display_config.ron");

    let game_data = GameDataBuilder::default()
        .with(
            PrefabLoaderSystem::<SpritePrefabData>::default(),
            "prefab_loader",
            &[],
        )
        .with_bundle(TransformBundle::new())?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                // The RenderToWindow plugin provides all the scaffolding for opening a window and drawing on it
                .with_plugin(
                    RenderToWindow::from_config_path(display_config)
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default()),
        )?;

    let mut game = Application::new(resources, state::MyState::default(), game_data)?;
    game.run();

    Ok(())
}
