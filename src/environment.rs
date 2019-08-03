use crate::{player::Direction, prefabs::TilePrefabs};

use amethyst::core::{
    components::Transform,
    ecs::prelude::*,
    math::{Unit, Vector2, Vector3},
};
use serde::{Deserialize, Serialize};
use specs_derive::Component;
use specs_static::{Id, Storage as StaticStorage};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TileMapId(pub u16, pub u16);

// Don't ask
impl Id for TileMapId {
    fn from_u32(value: u32) -> Self {
        // Ok since you're so nosy. This packs two u16's into one u32.
        // This code is adapted from https://gist.github.com/substack/525eaea52d0c7f52f2753c6fea28b9e8
        TileMapId((value % (1 << 16)) as u16, (value >> 16) as u16)
    }

    fn id(&self) -> u32 {
        // I have literally no idea if this breaks with big endian platforms.
        // Moxi tells me it doesn't so blame him if it does.
        let x = self.0.to_le_bytes();
        let y = self.1.to_le_bytes();
        u32::from_le_bytes([x[0], x[1], y[0], y[1]])
    }
}

pub type TileMap = StaticStorage<Tile, <Tile as Component>::Storage, TileMapId>;

/// Resource to track offset+scale of level's tiles from Transform 0,0 to index 0,0
#[derive(Default, Clone, Debug)]
pub struct LevelOffset(pub Transform);

pub fn attempt_bounce(
    from_transform: &Transform,
    on_map: &TileMap,
    with_offset: &LevelOffset,
) -> Option<Unit<Vector2<f32>>> {
    let transformed = from_transform.matrix() * with_offset.0.matrix();
    let index = (
        transformed.column(3).x as u16,
        transformed.column(3).y as u16,
    );

    if on_map
        .get(TileMapId(index.0, index.1 + 1))
        .map_or(false, |tile| *tile == Tile::Pusher)
    {
        Some((Direction::Up).to_unit_vector())
    } else if on_map
        .get(TileMapId(index.0, index.1 - 1))
        .map_or(false, |tile| *tile == Tile::Pusher)
    {
        Some((Direction::Down).to_unit_vector())
    } else if on_map
        .get(TileMapId(index.0 - 1, index.1))
        .map_or(false, |tile| *tile == Tile::Pusher)
    {
        Some((Direction::Left).to_unit_vector())
    } else if on_map
        .get(TileMapId(index.0 + 1, index.1))
        .map_or(false, |tile| *tile == Tile::Pusher)
    {
        Some((Direction::Right).to_unit_vector())
    } else {
        None
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Component)]
pub enum Tile {
    Nothing,
    Wall,
    Breakable,
    Pusher,
    Ball,
}

impl Tile {
    pub fn collides(self) -> bool {
        match self {
            Tile::Nothing | Tile::Ball => false,
            Tile::Wall | Tile::Breakable | Tile::Pusher => true,
        }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentConfig {
    pub map: String,
    pub start: (f32, f32),
    pub starting_direction: Direction,
    pub player_speed: (f32),
}

#[cfg(test)]
mod tests {
    use super::TileMapId;
    use specs_static::Id;

    #[test]
    fn are_my_u16s_screwed() {
        for x in 0..64 {
            for y in 0..64 {
                let id = TileMapId::from_u32(TileMapId(x, y).id());
                assert_eq!(id.0, x);
                assert_eq!(id.1, y);
            }
        }
    }
}

pub struct Line {
    pub start: Vector2<f32>,
    pub end: Vector2<f32>,

    pub direction: Unit<Vector2<f32>>,
    pub normal: Unit<Vector2<f32>>,
    pub length: f32,
}

impl Component for Line {
    type Storage = DenseVecStorage<Self>;
}

impl Line {
    pub fn new(start: Vector2<f32>, end: Vector2<f32>) -> Line {
        let diff = end - start;
        Line {
            start,
            end,
            direction: Unit::new_normalize(diff),
            normal: Unit::new_normalize(Vector2::new(-diff[0], diff[1])),
            length: diff.norm(),
        }
    }

    pub fn test_circle_intersection(&self, pos: &Vector2<f32>, radius: f32) -> bool {
        let v_start = pos - self.start;

        let dot_normal = v_start.dot(&self.normal);
        if dot_normal.abs() > radius {
            return false;
        }

        let dot_dir = v_start.dot(&self.direction);
        if dot_dir < -radius || dot_dir > self.length + radius {
            return false;
        } else if dot_dir > 0.0 && dot_dir < self.length {
            return true;
        }

        let sq_radius = radius * radius;
        if dot_dir < 0.0 && v_start.dot(&v_start) <= sq_radius {
            return true;
        } else {
            let v_end = pos - self.end;
            if v_end.dot(&v_end) <= sq_radius {
                return true;
            }
        }
        false
    }

    pub fn get_list_cells(&self, cell_size: f32) -> Vec<Vector2<u16>> {
        let mut start_point = self.start / cell_size;
        let start_cell = Vector2::new(start_point[0].floor() as u16, start_point[1].floor() as u16);
        let end_point = self.end / cell_size;
        let end_cell = Vector2::new(end_point[0].floor() as u16, end_point[1].floor() as u16);

        let mut cells = Vec::new();
        cells.push(start_cell);
        cells.push(end_cell);
        let direction = self.direction;

        for x in start_cell[0]..end_cell[0] {
            // direction[0] can't be 0 here since we move on X axis
            let y = start_point[1] + ((x as f32 - start_point[0]) / direction[0]) * direction[1];
            let y_cell = y.floor() as u16;
            cells.push(Vector2::new(x, y_cell));

            // Just to be safe. Could be unnecessary
            cells.push(Vector2::new(x, y_cell + 1));
        }
        for y in start_cell[1]..end_cell[1] {
            // direction[1] can't be 0 here since we move on X axis
            let x = start_point[0] + ((y as f32 - start_point[1]) / direction[1]) * direction[0];
            let x_cell = x.floor() as u16;
            cells.push(Vector2::new(x_cell, y));

            // Just to be safe. Could be unnecessary
            cells.push(Vector2::new(x_cell + 1, y));
        }
        cells
    }
}

pub fn create_line(world: &mut World, start: Vector2<f32>, end: Vector2<f32>) {
    let line_comp = Line::new(start, end);
    let scale = (end - start).norm() / 40.0;
    let dir = line_comp.direction;
    let angle = dir[1].atan2(dir[0]);
    let pos = (start + end) / 2.0;
    let mut transform = Transform::default();
    transform.set_rotation_2d(angle);
    transform.set_translation_xyz(pos[0], pos[1], 0.0);
    transform.set_scale(Vector3::new(scale, 2.0, 1.0));
    let wall_prefab = {
        let prefabs = world.read_resource::<TilePrefabs>();
        prefabs.get_prefab(Tile::Wall).unwrap().clone()
    };
    world
        .create_entity()
        .with(transform)
        .with(wall_prefab)
        .with(line_comp)
        .build();
}
