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
}

#[derive(Default, Copy, Clone, Debug)]
pub struct SpatialGridSystem;

impl<'a> System<'a> for SpatialGridSystem {
    type SystemData = (Entities<'a>, WriteStorage<'a, Line>, Write<'a, SpatialGrid>);

    fn run(&mut self, data: Self::SystemData) {
        let (entities, lines, mut spatial_grid) = data;
        spatial_grid.reset();
        for (entity, line) in (&entities, &lines).join() {
            let locations = line.get_list_cells(spatial_grid.cell_size());
            spatial_grid.add(entity, locations);
        }
    }
}
