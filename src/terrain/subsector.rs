use juno::{
    grid::{Grid, GridItem},
    ivec,
    vector::{self, IVec2, Vector},
};
use log::warn;

use crate::sector::{Sector, Tile};

use super::{
    structs::{GenTile, GenerationStage, Subsector},
    LONG_LENGTH, SHORT_LENGTH,
};

pub fn subsectors(size: &IVec2) -> Grid<Subsector> {
    let meta_size = ivec!(
        meta_size(*size.x() as u32) as i32,
        meta_size(*size.y() as u32) as i32
    );
    let mut meta_grid = Grid::<Subsector>::new(meta_size);
    for y in 0..*meta_grid.height() {
        for x in 0..*meta_grid.width() {
            let stage = generation_stage(&x, &y);
            let (width, height) = side_lengths(stage.clone());
            let (adjusted_width, adjusted_height) = adjust_lengths(stage.clone(), width, height);
            let subsector = Subsector::new(
                stage,
                Grid::<GenTile>::new(ivec!(adjusted_width as i32, adjusted_height as i32)),
            );
            let domsector = GridItem::new(ivec!(x, y), subsector);
            meta_grid.push(domsector);
        }
    }
    meta_grid
}

/// Calculate the size of the sector's "meta grid" —  that is, the size of the grid that contains
/// all subsectors.
/// > meta_size = 2 * ceil(size / (long + short)) - 1
/// Where:
///  - meta_size is the number of subsectors for that axis
///  - size is the total number of tiles for that axis
///  - long is the length of a large chunk size, in tiles
///  - short is the length of a short chunk size, in tiles
fn meta_size(size: u32) -> u32 {
    let size = size.max(1);
    2 * size.div_ceil(LONG_LENGTH + SHORT_LENGTH) - 1
}

// Calculate the position of a tile within it's sector
fn position_within_sector(mut position: u32) -> u32 {
    while position > LONG_LENGTH + SHORT_LENGTH {
        position -= LONG_LENGTH + SHORT_LENGTH
    }
    if position >= LONG_LENGTH {
        position - LONG_LENGTH
    } else {
        position
    }
}

/// Determines the generation stage of a subsector. Primary subsectors have even x and y meta coordinates.
/// Secondary chunks have a single even meta coordinate. Tertiary subsectors have exclusively odd
/// meta coordinates.
fn generation_stage(x: &i32, y: &i32) -> GenerationStage {
    match (x % 2 == 0, y % 2 == 0) {
        (true, true) => GenerationStage::Primary,
        (true, false) => GenerationStage::SecondaryHorizontal,
        (false, true) => GenerationStage::SecondaryVertical,
        (false, false) => GenerationStage::Tertiary,
    }
}

/// Determines the side length of a subchunk.
fn side_lengths(generation_stage: GenerationStage) -> (u32, u32) {
    match generation_stage {
        GenerationStage::Primary => (LONG_LENGTH, LONG_LENGTH),
        GenerationStage::SecondaryHorizontal => (LONG_LENGTH, SHORT_LENGTH),
        GenerationStage::SecondaryVertical => (SHORT_LENGTH, LONG_LENGTH),
        GenerationStage::Tertiary => (SHORT_LENGTH, SHORT_LENGTH),
    }
}

/// Adjust the actual lengths of subsectors to incorporate the size that they will eventually
/// become when they require additional size for blending
fn adjust_lengths(stage: GenerationStage, width: u32, height: u32) -> (u32, u32) {
    match stage {
        GenerationStage::Primary => (width, height),
        GenerationStage::SecondaryHorizontal => (width, height + 2),
        GenerationStage::SecondaryVertical => (width + 2, height),
        GenerationStage::Tertiary => (width + 2, height + 2),
    }
}

/// Expands a subsector, taking tiles from the meta grid. Adds symmtrically —  so an expansion of
/// (2, 1) will expand two tiles *on each side* horizontally, and one tile on each end vertically
fn expand_subsector(meta_grid: Grid<Subsector>, subsector_pos: &IVec2, expansion: IVec2) {
    let mut tiles_to_expand = Vec::with_capacity((2 * expansion.x() + 2 * expansion.y()) as usize);
    meta_grid.adjacent(subsector_pos).for_each(|adj_subsector| {
        match adj_subsector.pos() - subsector_pos {
            vector::DOWN => tiles_to_expand.append(&mut map_from_subsector(
                adj_subsector.contents(),
                ivec!(0, adj_subsector.contents().grid().height() - expansion.y()),
                ivec!(*adj_subsector.contents().grid().width(), 0),
                ivec!(*expansion.x(), 0),
            )),
            vector::LEFT => tiles_to_expand.append(&mut map_from_subsector(
                adj_subsector.contents(),
                ivec!(adj_subsector.contents().grid().width() - expansion.x(), 0),
                ivec!(
                    *adj_subsector.contents().grid().width(),
                    *adj_subsector.contents().grid().height()
                ),
                ivec!(0, *expansion.y()),
            )),
            _ => warn!("Found adjacent edge not in any cardinal direction"),
        }
    });
}

/// Take a rectangle from a subsector (from `min` to `max`), and return all coordinates within in,
/// mapped starting from the minimum corresponding to the `map_min` variable
fn map_from_subsector(
    subsector: &Subsector,
    min: IVec2,
    max: IVec2,
    map_min: IVec2,
) -> Vec<GridItem<GenTile>> {
    if min > max {
        warn!("Tried to map a subsector area with an inverted rectangle.");
    }
    subsector
        .grid()
        .tiles()
        .filter(|t| t.pos() >= &min && t.pos() <= &max)
        .map(|t| GridItem::new(t.pos() - &min + &map_min, t.contents().clone()))
        .collect::<Vec<GridItem<GenTile>>>()
}

pub fn stitch_subsectors(meta_grid: Grid<Subsector>, name: String, size: IVec2) -> Sector {
    let mut sector_grid = Grid::new(size);
    for y in 0..*sector_grid.height() {
        for x in 0..*sector_grid.width() {
            let (subsector, pos_in_subsector) = tile_position(ivec!(x, y));
            let error_tile = GridItem::new(ivec!(x, y), Tile::new(ivec!(21, 5), 1.));
            sector_grid.push(
                meta_grid
                    .tile(&subsector)
                    .map_or(error_tile.clone(), |subsector| {
                        subsector.contents().grid().tile(&pos_in_subsector).map_or(
                            error_tile,
                            |gen_tile| {
                                GridItem::new(
                                    ivec!(x, y),
                                    Tile::new(
                                        gen_tile
                                            .contents()
                                            .static_tile()
                                            .as_ref()
                                            .map_or(ivec!(22, 4), |static_tile| static_tile.pos()),
                                        1.,
                                    ),
                                )
                            },
                        )
                    }),
            )
        }
    }
    Sector::new(name, sector_grid, vec![])
}

/// Calculate the most up-to-date subsector and subsector coordinate of each tile
fn tile_position(pos: IVec2) -> (IVec2, IVec2) {
    let containing_subsector = ivec!(
        meta_size(*pos.x() as u32) as i32,
        meta_size(*pos.y() as u32) as i32
    );
    let position_within_sector = ivec!(
        position_within_sector(*pos.x() as u32) as i32,
        position_within_sector(*pos.y() as u32) as i32
    );
    (containing_subsector, position_within_sector)
}
