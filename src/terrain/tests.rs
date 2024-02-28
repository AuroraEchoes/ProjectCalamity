use cgmath::Vector2;
use log::info;
use rand::seq::SliceRandom;

use crate::juno::grid::Grid;

use super::{
    generate::{generate_subsector, select_tile},
    load_tilemap_json,
    structs::{GenTile, GenerationStage, Subsector},
};

#[test]
pub fn test_subsector_generation() {
    let tiles = load_tilemap_json();
    let mut subsector = Subsector::new(GenerationStage::Primary, Grid::new(Vector2::new(8, 8)));
    subsector.grid_mut().fill(GenTile::empty());
    generate_subsector(&mut subsector, tiles.as_slice());
    let empty_count = subsector
        .grid()
        .tiles()
        .filter(|t| t.contents().static_tile().is_none())
        .count();
    info!("Test Subsector Contents: {:?}", subsector);
    assert_eq!(empty_count, 0);
}

#[test]
pub fn test_tile_allowed() {
    let mut rand = rand::thread_rng();
    let tiles = load_tilemap_json();
    let mut grid = Grid::new(Vector2::new(3, 3));
    grid.fill(GenTile::empty());
    // We'll do top and bottom, and left and right, seperately.

    // Top
    let top_tile = tiles.choose(&mut rand).unwrap();
    grid.tile_mut(Vector2 { x: 1, y: 2 })
        .unwrap()
        .contents_mut()
        .set_static_tile(top_tile.clone());
    let up = top_tile.down();
    // Bottom
    let bottom_tile = tiles.choose(&mut rand).unwrap();
    grid.tile_mut(Vector2 { x: 1, y: 0 })
        .unwrap()
        .contents_mut()
        .set_static_tile(bottom_tile.clone());
    let down = bottom_tile.up();
    select_tile(&mut grid, Vector2::new(1, 1), tiles.as_slice());
    let chosen_static = grid
        .tile(Vector2::new(1, 1))
        .unwrap()
        .contents()
        .static_tile()
        .as_ref()
        .unwrap();
    assert_eq!(chosen_static.up(), up);
    assert_eq!(chosen_static.down(), down);

    grid.fill(GenTile::empty());
    // Left
    let left_tile = tiles.choose(&mut rand).unwrap();
    grid.tile_mut(Vector2 { x: 0, y: 1 })
        .unwrap()
        .contents_mut()
        .set_static_tile(left_tile.clone());
    let left = left_tile.right();
    // Right
    let right_tile = tiles.choose(&mut rand).unwrap();
    grid.tile_mut(Vector2 { x: 2, y: 1 })
        .unwrap()
        .contents_mut()
        .set_static_tile(right_tile.clone());
    let right= right_tile.left();
    select_tile(&mut grid, Vector2::new(1, 1), tiles.as_slice());
    let chosen_static = grid
        .tile(Vector2::new(1, 1))
        .unwrap()
        .contents()
        .static_tile()
        .as_ref()
        .unwrap();
    assert_eq!(chosen_static.left(), left);
    assert_eq!(chosen_static.right(), right)

}
