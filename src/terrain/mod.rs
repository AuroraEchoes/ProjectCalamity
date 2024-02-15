use cgmath::Vector2;
use log::info;

use crate::sector::Sector;

use self::{
    generate::{generate_primary_sectors, generate_secondary_sectors, generate_tertiary_sectors},
    structs::StaticTileInfo,
    subsector::{stitch_subsectors, subsectors},
};

pub mod generate;
pub mod structs;
pub mod subsector;

// Parameters
// TODO: Maybe make this into a struct?
const BASE_SIZE: u32 = 2; // The smallest size used
const LONG_RATIO: u32 = 4; // Ratio between base size and "long" size
const SHORT_LENGTH: u32 = BASE_SIZE;
const LONG_LENGTH: u32 = BASE_SIZE * LONG_RATIO;

pub fn generate_terrain(size: Vector2<u32>, name: String) -> Sector {
    // For now let's put in some pseudocode!
    // 1. Find the primary sectors
    let static_tiles_vec = load_tilemap_json();
    let static_tiles = static_tiles_vec.as_slice();
    let mut meta_grid = subsectors(size);
    info!("Created meta grid");
    // 2. Fill each primary subsector randomly using wave function collapse —  each subsector on a
    //    seperate thread
    generate_primary_sectors(&mut meta_grid, static_tiles);
    info!("Generated primary sectors");
    // 3. Find secondary sectors, including the borders from the the primary sectors *but not* any
    //    tiles from tertiary sectors
    // 4. Fill each secondary sector randomly using wave function collapse —  each subsector on a
    //    seperate thread
    generate_secondary_sectors(&mut meta_grid, static_tiles);
    // 5. ...(tertiary sectors)
    generate_tertiary_sectors(&mut meta_grid, static_tiles);
    info!("Generated all sectors");
    // 6. Combine all of the disperate subsectors, resolving overlapping tiles and prioritising
    //    later sectors for overlaps (figure out which subsector should "own" the tile, I.E. the
    //    latest subsector which will contain this tile)
    stitch_subsectors(meta_grid, name, size)
}

// HACK: This is temporary, until I write a proper asset loading system
pub fn load_tilemap_json() -> Vec<StaticTileInfo> {
    let json = include_str!("../../assets/tilemap.json");
    let static_tiles: Vec<StaticTileInfo> = serde_json::from_str(json).unwrap();
    static_tiles
}
