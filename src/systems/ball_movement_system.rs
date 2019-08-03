use amethyst::core::{
    ecs::prelude::*,
    math::{Unit, Vector3},
    Transform,
};

use crate::{
    environment::{LevelOffset, Line, TileMap, TileMapId},
    player::Ball,
    systems::SpatialGrid,
};

#[derive(Default, Copy, Clone, Debug)]
pub struct BallMovementSystem;

impl<'a> System<'a> for BallMovementSystem {
    type SystemData = (
        WriteStorage<'a, Ball>,
        WriteStorage<'a, Transform>,
        ReadStorage<'a, Line>,
        Read<'a, SpatialGrid>,
        Read<'a, TileMap>,
        Read<'a, LevelOffset>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut balls, mut transforms, lines, spatial_grid, tile_map, level_offset) = data;

        for (mut ball, transform) in (&mut balls, &mut transforms).join() {
            let intended_point = (transform.matrix() * level_offset.0.matrix())
                .column(3)
                .xy()
                + ball.direction.into_inner() * ball.velocity;
            let query = spatial_grid.query(&intended_point);

            for (line, _) in (&lines, &query).join() {
                if line.test_circle_intersection(&intended_point, ball.radius) {
                    let ball_dir = ball.direction.clone().into_inner();
                    let normal = line.normal.clone().into_inner();
                    ball.direction =
                        Unit::new_normalize(ball_dir - 2.0 * ball_dir.dot(&normal) * normal);
                }
            }
            transform.append_translation(Vector3::new(ball.direction.x, ball.direction.y, 0.0));
        }
    }
}
