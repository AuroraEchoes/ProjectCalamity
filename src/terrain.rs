use rand::Rng;

use crate::{
    sector::{Sector, Tile},
    utility::GridPosVec,
};

pub fn generate_terrain(name: String, size: GridPosVec) -> Sector {
    let mut random = rand::thread_rng();
    let mut tiles = Vec::with_capacity(size.x() * size.y());
    for y in 0..size.y() {
        for x in 0..size.x() {
            let tile = TileType::from_index(random.gen_range(0..2));
            tiles.push(Tile::new(GridPosVec::new(x, y), tile, 1.0));
        }
    }
    let sector = Sector::new(name, size, tiles, Vec::new());
    return sector;
}

#[derive(PartialEq)]
pub enum TileType {
    Ground,
    Water,
}

impl TileType {
    fn from_index(index: i32) -> Self {
        match index {
            0 => TileType::Ground,
            1 => TileType::Water,
            _ => TileType::Ground,
        }
    }
}
