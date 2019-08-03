use amethyst::core::{ecs::prelude::*, math::Vector3, Transform};

use crate::{
    environment::{LevelOffset, TileMap, TileMapId},
    player::Ball,
};

#[derive(Default, Copy, Clone, Debug)]
pub struct BallMovementSystem;

impl<'a> System<'a> for BallMovementSystem {
    type SystemData = (
        WriteStorage<'a, Ball>,
        WriteStorage<'a, Transform>,
        Read<'a, TileMap>,
        Read<'a, LevelOffset>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut balls, mut transforms, tile_map, level_offset) = data;

        for (mut ball, transform) in (&mut balls, &mut transforms).join() {
            let intended_point = (transform.matrix() * level_offset.0.matrix())
                .column(3)
                .xy()
                + ball.direction.into_inner() * ball.velocity;

            if tile_map
                .get(TileMapId(intended_point.x as u16, intended_point.y as u16))
                .map_or(false, |tile| tile.collides())
            {
                ball.direction = -ball.direction;
            }

            // Future collision testing could be done here based on normals
            // and essentially an AABB test with an added radius for the circle.

            transform.append_translation(Vector3::new(ball.direction.x, ball.direction.y, 0.0));
        }
    }
}
