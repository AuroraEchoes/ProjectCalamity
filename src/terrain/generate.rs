use juno::{
    grid::{Grid, GridItem},
    ivec,
    vector::{self, IVec2, Vector},
};
use log::{info, warn};
use rand::seq::SliceRandom;

use super::structs::{GenTile, GenerationStage, StaticTileInfo, Subsector};

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
        let grid_item = GridItem::new(ivec!(0, 0), GenTile::new(initial_tile.clone(), None));
        if let Some(t) = subsector.grid_mut().tile_mut(&ivec!(0, 0)) {
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
        let grid_item = GridItem::new(ivec!(0, 0), GenTile::new(initial_tile.clone(), None));
        if let Some(t) = subsector.grid_mut().tile_mut(&ivec!(0, 0)) {
            *t = grid_item
        }
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
        let grid_item = GridItem::new(ivec!(0, 0), GenTile::new(initial_tile.clone(), None));
        if let Some(t) = subsector.grid_mut().tile_mut(&ivec!(0, 0)) {
            *t = grid_item
        }
        generate_subsector(subsector, static_tiles)
    }
}

fn generate_subsector(subsector: &mut Subsector, static_tiles: &[StaticTileInfo]) {
    while let Some(sel_tile) = min_entropy(&subsector.grid().clone()) {
        info!("Selected tile {:?}", sel_tile);
        select_tile(subsector.grid_mut(), sel_tile, static_tiles);
        calculate_entropy(subsector.grid_mut(), sel_tile, static_tiles);
    }
}

fn select_tile(grid: &mut Grid<GenTile>, pos: &IVec2, static_tiles: &[StaticTileInfo]) {
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

fn choose_tile(pos: &IVec2, possible_tiles: &[StaticTileInfo]) -> StaticTileInfo {
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

fn tile_allowed(pos: &IVec2, stat: &StaticTileInfo, grid: &Grid<GenTile>) -> bool {
    // Filtering for edges which *don't* allow this tile. So if this has a length that
    // isn't 0, we know that we can't use this tile

    grid.adjacent(pos)
        .map(|adj_tile| match adj_tile.pos() - pos {
            vector::DOWN => static_tile(adj_tile).map_or(true, |t| t.up().eq(stat.down())),
            vector::LEFT => static_tile(adj_tile).map_or(true, |t| t.right().eq(stat.left())),
            vector::RIGHT => static_tile(adj_tile).map_or(true, |t| t.left().eq(stat.right())),
            vector::UP => static_tile(adj_tile).map_or(true, |t| t.down().eq(stat.up())),
            _ => {
                warn!("Found adjacent edge not in any cardinal direction");
                false
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

fn calculate_entropy(grid: &mut Grid<GenTile>, pos: &IVec2, static_tiles: &[StaticTileInfo]) {
    let positions = grid
        .adjacent(pos)
        .map(|t| t.pos().clone())
        .collect::<Vec<_>>();
    positions.iter().for_each(|pos| {
        let entropy = static_tiles
            .iter()
            .filter(|stat| tile_allowed(pos, stat, grid))
            .count();
        if let Some(tile) = grid.tile_mut(pos) {
            let contents = tile.contents_mut();
            if !contents.tile_set() {
                tile.contents_mut().set_entropy(entropy as u32);
            }
        }
    });
}

fn min_entropy(grid: &Grid<GenTile>) -> Option<&IVec2> {
    let mut min_entropy = None::<&GridItem<GenTile>>;
    for t in grid.tiles() {
        let prev = min_entropy.map_or(u32::MAX, |t| t.contents().entropy().unwrap_or(u32::MAX));
        let curr = t.contents().entropy().unwrap_or(u32::MAX);
        if curr < prev {
            min_entropy = Some(t);
        }
    }
    match min_entropy {
        Some(item) => Some(item.pos()),
        None => None,
    }
}
