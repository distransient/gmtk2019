use amethyst::{
    core::{components::Transform, ecs::prelude::*},
    input::{InputHandler, StringBindings, VirtualKeyCode},
};

use crate::{
    environment::{attempt_bounce, LevelOffset, TileMap},
    player::Ball,
};

#[derive(Default, Copy, Clone, Debug)]
pub struct BallControlSystem;

impl<'a> System<'a> for BallControlSystem {
    type SystemData = (
        WriteStorage<'a, Ball>,
        ReadStorage<'a, Transform>,
        Read<'a, TileMap>,
        Read<'a, LevelOffset>,
        Read<'a, InputHandler<StringBindings>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut balls, transforms, tile_map, level_offset, input) = data;

        if input.key_is_down(VirtualKeyCode::Space) {
            for (mut ball, transform) in (&mut balls, &transforms).join() {
                if let Some(direction) = attempt_bounce(&transform, &tile_map, &level_offset) {
                    ball.direction = direction;
                }
            }
        }
    }
}
