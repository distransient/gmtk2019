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
                    // This is temporary, for debugging purpose.
                    // To properly handle the collision, we need to check that the normal goes in the right direction.
                    let ball_dir = ball.direction.clone().into_inner();
                    let normal = line.normal.clone().into_inner();
                    ball.direction =
                        Unit::new_normalize(ball_dir - 2.0 * ball_dir.dot(&normal) * normal);
                }
            }

            //
            //            if tile_map
            //                .get(TileMapId(intended_point.x as u16, intended_point.y as u16))
            //                .map_or(false, |tile| tile.collides())
            //            {
            //                ball.direction = -ball.direction;
            //            }

            // Future collision testing could be done here based on normals
            // and essentially an AABB test with an added radius for the circle.

            transform.append_translation(Vector3::new(ball.direction.x, ball.direction.y, 0.0));
        }
    }
}
