use cgmath::Vector2;
use log::info;
use rand::seq::SliceRandom;

use crate::juno::{
    directions,
    grid::{Grid, GridItem},
};

use super::structs::{Entropy, GenTile, GenerationStage, StaticTileInfo, Subsector};

pub fn generate_primary_sectors(meta_grid: &mut Grid<Subsector>, static_tiles: &[StaticTileInfo]) {
    let primary_sectors = meta_grid.tiles_mut().filter(|t| {
        t.contents()
            .generation_stage()
            .eq(&GenerationStage::Primary)
    });
    for subsector in primary_sectors {
        // Primary sectors need a tile set initially
        // HACK: do proper stuff for tertiary sectors
        let subsector = subsector.contents_mut();
        subsector.grid_mut().fill(GenTile::empty());
        let initial_tile = static_tiles.choose(&mut rand::thread_rng()).unwrap();
        let grid_item = GridItem::new(Vector2::new(0, 0), GenTile::new(initial_tile.clone()));
        if let Some(t) = subsector.grid_mut().tile_mut(Vector2::new(0, 0)) {
            *t = grid_item
        }
        generate_subsector(subsector, static_tiles)
    }
}

pub fn generate_secondary_sectors(
    meta_grid: &mut Grid<Subsector>,
    static_tiles: &[StaticTileInfo],
) {
    let secondary_sectors = meta_grid.tiles_mut().filter(|t| {
        t.contents().generation_stage() == &GenerationStage::SecondaryVertical
            || t.contents().generation_stage() == &GenerationStage::SecondaryHorizontal
    });
    for subsector in secondary_sectors {
        // Primary sectors need a tile set initially
        // HACK: do proper stuff for tertiary sectors
        let subsector = subsector.contents_mut();
        subsector.grid_mut().fill(GenTile::empty());
        let initial_tile = static_tiles.choose(&mut rand::thread_rng()).unwrap();
        let grid_item = GridItem::new(Vector2::new(0, 0), GenTile::new(initial_tile.clone()));
        if let Some(t) = subsector.grid_mut().tile_mut(Vector2::new(0, 0)) {
            *t = grid_item;
        }
        calculate_entropy(subsector.grid_mut(), Vector2::new(0, 0), static_tiles);
        generate_subsector(subsector, static_tiles)
    }
}

pub fn generate_tertiary_sectors(meta_grid: &mut Grid<Subsector>, static_tiles: &[StaticTileInfo]) {
    let tertiary_sectors = meta_grid.tiles_mut().filter(|t| {
        t.contents()
            .generation_stage()
            .eq(&GenerationStage::Tertiary)
    });
    for subsector in tertiary_sectors {
        // Primary sectors need a tile set initially
        // HACK: do proper stuff for tertiary sectors
        let subsector = subsector.contents_mut();
        subsector.grid_mut().fill(GenTile::empty());
        let initial_tile = static_tiles.choose(&mut rand::thread_rng()).unwrap();
        let grid_item = GridItem::new(Vector2::new(0, 0), GenTile::new(initial_tile.clone()));
        if let Some(t) = subsector.grid_mut().tile_mut(Vector2::new(0, 0)) {
            *t = grid_item
        }
        generate_subsector(subsector, static_tiles)
    }
}

pub fn generate_subsector(subsector: &mut Subsector, static_tiles: &[StaticTileInfo]) {
    while count_empty_tiles(&subsector) > 0 {
        if let Some(sel_tile) = min_entropy(&subsector.grid().clone()) {
            select_tile(subsector.grid_mut(), sel_tile, static_tiles);
            calculate_entropy(subsector.grid_mut(), sel_tile, static_tiles);
        }
        // We need to pick a new untouched tile and set it's entropy
        else {
            calculate_entropy_for_empty_tile(subsector.grid_mut(), static_tiles);
        }
    }
}

fn count_empty_tiles(subsector: &Subsector) -> u32 {
    subsector
        .grid()
        .tiles()
        .filter(|t| t.contents().static_tile().is_none())
        .count() as u32
}

fn calculate_entropy_for_empty_tile(
    subsector: &mut Grid<GenTile>,
    static_tiles: &[StaticTileInfo],
) {
    let empty_tile_opt = subsector
        .tiles()
        .filter(|t| t.contents().entropy().eq(&Entropy::Uncalulated))
        .nth(0)
        .cloned();
    if let Some(empty_tile) = empty_tile_opt {
        select_tile(subsector, empty_tile.pos(), static_tiles);
        calculate_entropy(subsector, empty_tile.pos(), static_tiles);
    }
}

pub fn select_tile(grid: &mut Grid<GenTile>, pos: Vector2<u32>, static_tiles: &[StaticTileInfo]) {
    // Find possible adjacent tiles
    // 1. Filter static tiles with regard to edges
    // 2. `choose_tile()` from the filtered list
    let possible_tiles = static_tiles
        .iter()
        .filter(|stat| tile_allowed(pos, stat, grid))
        .cloned()
        .collect::<Vec<_>>();
    if let Some(tile) = grid.tile_mut(pos) {
        let chosen = choose_tile(pos, possible_tiles.as_slice());
        tile.contents_mut().set_static_tile(chosen);
        tile.contents_mut().remove_entropy();
    }
}

fn choose_tile(pos: Vector2<u32>, possible_tiles: &[StaticTileInfo]) -> StaticTileInfo {
    match possible_tiles.choose(&mut rand::thread_rng()) {
        Some(tile) => tile.clone(),
        // Default red error tile
        None => StaticTileInfo::new(
            22,
            4,
            "grass".to_string(),
            "grass".to_string(),
            "grass".to_string(),
            "grass".to_string(),
        ),
    }
}

fn tile_allowed(pos: Vector2<u32>, stat: &StaticTileInfo, grid: &Grid<GenTile>) -> bool {
    // Filtering for edges which *don't* allow this tile. So if this has a length that
    // isn't 0, we know that we can't use this tile
    let pos_i32 = Vector2::new(pos.x as i32, pos.y as i32);

    grid.adjacent(pos)
        .map(|adj_tile| {
            match Vector2::new(adj_tile.pos().x as i32, adj_tile.pos().y as i32) - pos_i32 {
                directions::DOWN => static_tile(adj_tile).map_or(true, |t| t.up().eq(stat.down())),
                directions::LEFT => {
                    static_tile(adj_tile).map_or(true, |t| t.right().eq(stat.left()))
                }
                directions::RIGHT => {
                    static_tile(adj_tile).map_or(true, |t| t.left().eq(stat.right()))
                }
                directions::UP => static_tile(adj_tile).map_or(true, |t| t.down().eq(stat.up())),
                _ => false,
            }
        })
        .filter(|b| !b) // We're checking for non-matching edges
        .count()
        .eq(&0) // All of the edges match
}

/// Helper function to get the Option<&StaticTileInfo> in less characters
fn static_tile(adj_tile: &GridItem<GenTile>) -> Option<&StaticTileInfo> {
    adj_tile.contents().static_tile().as_ref()
}

fn calculate_entropy(grid: &mut Grid<GenTile>, pos: Vector2<u32>, static_tiles: &[StaticTileInfo]) {
    let positions = grid
        .adjacent(pos)
        .map(|t| t.pos().clone())
        .collect::<Vec<_>>();
    positions.iter().for_each(|pos| {
        let entropy = static_tiles
            .iter()
            .filter(|stat| tile_allowed(*pos, stat, grid))
            .count();
        if let Some(tile) = grid.tile_mut(*pos) {
            let contents = tile.contents_mut();
            if !contents.tile_set() {
                tile.contents_mut().set_entropy(entropy as u32);
            }
        }
    });
}

fn min_entropy(grid: &Grid<GenTile>) -> Option<Vector2<u32>> {
    let mut min_entropy_tile = None::<&GridItem<GenTile>>;
    let mut min_entropy_value = u32::MAX;
    for t in grid.tiles() {
        match t.contents().entropy() {
            Entropy::Uncalulated => {}
            Entropy::Calculated(e) => {
                if e < min_entropy_value {
                    min_entropy_value = e;
                    min_entropy_tile = Some(t);
                }
            }
            Entropy::Set => {}
        };
    }
    match min_entropy_tile {
        Some(item) => Some(item.pos()),
        None => None,
    }
}
