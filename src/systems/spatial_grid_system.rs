use amethyst::core::{ecs::prelude::*, math::Vector2, Transform};

use crate::{environment::Line, player::Ball};

use std::collections::HashMap;
use std::u16;

pub struct SpatialGrid {
    cell_size: f32,
    cells: HashMap<u32, BitSet>,
}

impl Default for SpatialGrid {
    fn default() -> Self {
        SpatialGrid {
            cell_size: 40f32,
            cells: HashMap::new(),
        }
    }
}

impl SpatialGrid {
    pub fn reset(&mut self) {
        self.cells = HashMap::new();
    }

    pub fn cell_size(&self) -> f32 {
        self.cell_size
    }
}

impl SpatialGrid {
    pub fn add(&mut self, entity: Entity, locations: Vec<Vector2<u16>>) {
        for loc in locations {
            let ind = loc[0] as u32 + (u16::MAX as u32) * (loc[1] as u32);
            let mut set = self.cells.entry(ind).or_insert(BitSet::new());
            set.add(entity.id());
        }
    }

    pub fn query(&self, query_position: Vector2<f32>) -> BitSet {
        let mut query_result = BitSet::new();
        let query_cell = Vector2::new((query_position[0] / self.cell_size).floor() as u16, (query_position[1] / self.cell_size).floor() as u16);
        let min_x = if query_cell[0] > 0 {
            query_cell[0] - 1
        } else {
            0u16
        };
        let min_y = if query_cell[1] > 0 {
            query_cell[1] - 1
        } else {
            0u16
        };
        for x in min_x..query_cell[0] + 1 {
            for y in min_y..query_cell[1] + 1 {
                let ind = x as u32 + (u16::MAX as u32) * (y as u32);
                match self.cells.get(&ind) {
                    Some(bs) => query_result |= bs,
                    None => (),
                }
            }
        }
        query_result
    }
}

#[derive(Default, Copy, Clone, Debug)]
pub struct SpatialGridSystem;

impl<'a> System<'a> for SpatialGridSystem {
    type SystemData = (Entities<'a>, ReadStorage<'a, Line>, Write<'a, SpatialGrid>);

    fn run(&mut self, data: Self::SystemData) {
        let (entities, lines, mut spatial_grid) = data;
        spatial_grid.reset();
        for (entity, line) in (&entities, &lines).join() {
            let locations = line.get_list_cells(spatial_grid.cell_size());
            spatial_grid.add(entity, locations);
        }
    }
}
