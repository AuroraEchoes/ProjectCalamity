use cgmath::Vector2;

use crate::{
    juno::{
        directions,
        grid::{Grid, GridItem},
    },
    sector::{Sector, Tile},
};

use super::{
    structs::{GenTile, GenerationStage, Subsector},
    LONG_LENGTH, SHORT_LENGTH,
};

pub fn subsectors(size: Vector2<u32>) -> Grid<Subsector> {
    let meta_size = Vector2::new(meta_size(size.x), meta_size(size.y));
    let mut meta_grid = Grid::<Subsector>::new(meta_size);
    for y in 0..meta_grid.height() {
        for x in 0..meta_grid.width() {
            let stage = generation_stage(x, y);
            let (width, height) = side_lengths(stage.clone());
            let (adjusted_width, adjusted_height) = adjust_lengths(stage.clone(), width, height);
            let subsector = Subsector::new(
                stage,
                Grid::<GenTile>::new(Vector2::new(adjusted_width, adjusted_height)),
            );
            let domsector = GridItem::new(Vector2::new(x, y), subsector);
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
fn generation_stage(x: u32, y: u32) -> GenerationStage {
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
fn expand_subsector(
    meta_grid: Grid<Subsector>,
    subsector_pos: Vector2<u32>,
    expansion: Vector2<u32>,
) {
    let mut tiles_to_expand = Vec::with_capacity((2 * expansion.x + 2 * expansion.y) as usize);
    meta_grid.adjacent(subsector_pos).for_each(|adj_subsector| {
        let adj_subsector_pos =
            Vector2::new(adj_subsector.pos().x as i32, adj_subsector.pos().y as i32);
        let subsector_pos_i32 = Vector2::new(subsector_pos.x as i32, subsector_pos.y as i32);
        // TODO: Take a look at whether I'm missing the other sides.
        match adj_subsector_pos - subsector_pos_i32 {
            directions::DOWN => tiles_to_expand.append(&mut map_from_subsector(
                adj_subsector.contents(),
                Vector2::new(0, adj_subsector.contents().grid().height() - expansion.y),
                Vector2::new(adj_subsector.contents().grid().width(), 0),
                Vector2::new(expansion.x, 0),
            )),
            directions::LEFT => tiles_to_expand.append(&mut map_from_subsector(
                adj_subsector.contents(),
                Vector2::new(adj_subsector.contents().grid().width() - expansion.x, 0),
                Vector2::new(
                    adj_subsector.contents().grid().width(),
                    adj_subsector.contents().grid().height(),
                ),
                Vector2::new(0, expansion.y),
            )),
            _ => println!("Found adjacent edge not in any cardinal direction"),
        }
    });
}

/// Take a rectangle from a subsector (from `min` to `max`), and return all coordinates within in,
/// mapped starting from the minimum corresponding to the `map_min` variable
fn map_from_subsector(
    subsector: &Subsector,
    min: Vector2<u32>,
    max: Vector2<u32>,
    map_min: Vector2<u32>,
) -> Vec<GridItem<GenTile>> {
    subsector
        .grid()
        .tiles()
        .filter(|t| {
            (t.pos().x >= min.x && t.pos().y >= min.y) && (t.pos().x <= max.x && t.pos().y <= max.y)
        })
        .map(|t| GridItem::new(t.pos() - &min + &map_min, t.contents().clone()))
        .collect::<Vec<GridItem<GenTile>>>()
}

pub fn stitch_subsectors(meta_grid: Grid<Subsector>, name: String, size: Vector2<u32>) -> Sector {
    let mut sector_grid = Grid::new(size);
    for y in 0..sector_grid.height() {
        for x in 0..sector_grid.width() {
            let (subsector, pos_in_subsector) = tile_position(Vector2::new(x, y));
            let error_tile = GridItem::new(Vector2::new(x, y), Tile::new(Vector2::new(21, 5), 1.));
            sector_grid.push(
                meta_grid
                    .tile(subsector)
                    .map_or(error_tile.clone(), |subsector| {
                        subsector.contents().grid().tile(pos_in_subsector).map_or(
                            error_tile,
                            |gen_tile| {
                                GridItem::new(
                                    Vector2::new(x, y),
                                    Tile::new(
                                        gen_tile.contents().static_tile().as_ref().map_or(
                                            Vector2::new(22, 4),
                                            |static_tile| {
                                                Vector2::new(
                                                    static_tile.pos().x as u32,
                                                    static_tile.pos().y as u32,
                                                )
                                            },
                                        ),
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
fn tile_position(pos: Vector2<u32>) -> (Vector2<u32>, Vector2<u32>) {
    let containing_subsector = Vector2::new(meta_size(pos.x), meta_size(pos.y));
    let position_within_sector =
        Vector2::new(position_within_sector(pos.x), position_within_sector(pos.y));
    (containing_subsector, position_within_sector)
}
