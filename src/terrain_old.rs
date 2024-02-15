// For now, we'll do a simple wave-function collapse implementation
// Later, we can add the special sauce

use cgmath::Vector2;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

use crate::{
    juno::{
        directions,
        grid::{Grid, GridItem},
    },
    sector::{Sector, Tile},
};

const MAX_CHUNK_LENGTH: u32 = 8;
const CHUNK_BLEND_LENGTH: u32 = 2;

pub fn generate_terrain(dimensions: Vector2<u32>, name: String) -> Sector {
    let mut generation_grid = Grid::<TerrainGenerationTile>::new(dimensions);
    let static_tiles = load_tilemap_json();
    generation_grid.fill(TerrainGenerationTile::new());

    // Set starting position
    // NOTE: consider doing this randomly in future?
    let starting_position = Vector2::new(0, 0);
    calculate_tile(&mut generation_grid, starting_position, &static_tiles);
    calculate_adj_entropies(&mut generation_grid, starting_position, &static_tiles);
    let mut previous_empty_tiles = 0;
    let mut empty_tiles_stored = empty_tiles(&generation_grid);
    while empty_tiles_stored > 0 {
        println!("Remaining {empty_tiles_stored:?}");
        if let Some(selected_position) = min_entropy(&generation_grid.clone()) {
            calculate_tile(&mut generation_grid, selected_position, &static_tiles);
            if previous_empty_tiles == empty_tiles_stored {
                // We wanna set and forget
                let tile = generation_grid.tile_mut(selected_position).unwrap();
                tile.contents_mut().set_tile(StaticTileInfo {
                    x: 22,
                    y: 4,
                    down: "grass".to_string(),
                    left: "grass".to_string(),
                    up: "grass".to_string(),
                    right: "grass".to_string(),
                });
                tile.contents_mut().remove_entropy();
            }
            calculate_adj_entropies(&mut generation_grid, selected_position, &static_tiles);
        } else {
        }
        previous_empty_tiles = empty_tiles_stored;
        empty_tiles_stored = empty_tiles(&generation_grid);
    }

    generation_grid_to_sector(name, generation_grid)
}

fn divide_to_subsectors<'a>(size: Vector2<i32>) -> Grid<MTGenSubSector<'a>> {
    let meta_size = Vector2::new(
        2 * (size.x as u32).div_ceil(MAX_CHUNK_LENGTH + CHUNK_BLEND_LENGTH) - 1,
        2 * (size.y as u32).div_ceil(MAX_CHUNK_LENGTH + CHUNK_BLEND_LENGTH) - 1,
    );
    let mut meta_gen_grid = Grid::<MTGenSubSector>::new(meta_size);
    for y in 0..meta_gen_grid.height() {
        for x in 0..meta_gen_grid.width() {
            // Large "primary" subsectors will always have even coordinates, while secondary
            // sections will have one even and one odd coordinate, and tertiary sections will have
            // exclusively odd coordinates
            let width = if x % 2 == 0 {
                MAX_CHUNK_LENGTH
            } else {
                CHUNK_BLEND_LENGTH
            };
            let height = if y % 2 == 0 {
                MAX_CHUNK_LENGTH
            } else {
                CHUNK_BLEND_LENGTH
            };
            let stage = match (width == MAX_CHUNK_LENGTH, height == MAX_CHUNK_LENGTH) {
                (true, true) => GenerationStage::Primary,
                (true, false) | (false, true) => GenerationStage::Secondary,
                (false, false) => GenerationStage::Tertiary,
            };
            let subsector =
                MTGenSubSector::new(stage, Grid::<MTGenTile>::new(Vector2::new(width, height)));
            let domsector = GridItem::new(Vector2::new(x, y), subsector);
            meta_gen_grid.push(domsector);
        }
    }
    meta_gen_grid
}

fn test_meta_sec(meta_grid: Grid<MTGenSubSector>, name: String, size: Vector2<u32>) -> Sector {
    let mut sector_grid = Grid::new(size.clone());
    sector_grid.fill(Tile::new(Vector2::new(0, 0), 1.));
    for subsector in meta_grid.tiles() {
        let sub_x = (0..subsector.pos().x).fold(0, |acc, t| {
            acc + match t % 2 == 0 {
                true => MAX_CHUNK_LENGTH,
                false => CHUNK_BLEND_LENGTH,
            }
        });
        let sub_y = (0..subsector.pos().y).fold(0, |acc, t| {
            acc + match t % 2 == 0 {
                true => MAX_CHUNK_LENGTH,
                false => CHUNK_BLEND_LENGTH,
            }
        });
        for tile in subsector.contents().tiles().tiles() {
            if let Some(old_tile) =
                sector_grid.tile_mut(Vector2::new(sub_x + tile.pos().x, sub_y + tile.pos().y))
            {
                *old_tile = GridItem::new(
                    Vector2::new(sub_x + tile.pos().x, sub_y + tile.pos().y),
                    Tile::new(tile.contents().static_tile.pos(), 1.),
                );
            }
        }
    }
    Sector::new(name, sector_grid, vec![])
}

fn generate_chunk(chunk: &mut MTGenSubSector) {
    // If there are no adjacent tiles, we start from scratch. If there *are* adjacent tiles,
}

fn meta_grid_to_sector(
    meta_grid: Grid<MTGenSubSector>,
    name: String,
    size: Vector2<u32>,
) -> Sector {
    let mut sector_grid = Grid::new(size);
    for y in 0..size.y {
        for x in 0..size.x {
            let subsector_pos = Vector2::new(
                2 * (x as u32).div_ceil(MAX_CHUNK_LENGTH + CHUNK_BLEND_LENGTH) - 1,
                2 * (y as u32).div_ceil(MAX_CHUNK_LENGTH + CHUNK_BLEND_LENGTH) - 1,
            );
            let sub_x = (0..subsector_pos.x).fold(x, |acc, t| {
                acc - match t % 2 == 0 {
                    true => 8,
                    false => 2,
                }
            });
            let sub_y = (0..subsector_pos.x).fold(y, |acc, t| {
                acc - match t % 2 == 0 {
                    true => 8,
                    false => 2,
                }
            });
            if let Some(subsector) = meta_grid.tile(subsector_pos) {
                if let Some(tile) = subsector
                    .contents()
                    .tiles()
                    .tile(Vector2::new(sub_x, sub_y))
                {
                    let sector_tile = Tile::new(tile.contents().static_tile.pos(), 1.);
                    sector_grid.push(GridItem::new(Vector2::new(x, y), sector_tile));
                } else {
                }
            } else {
            }
        }
    }
    Sector::new(name, sector_grid, vec![])
}

// Tile for multithreading generation
#[derive(Clone, Debug)]
pub struct MTGenTile<'a> {
    // Contents
    static_tile: &'a StaticTileInfo,
}

#[derive(Clone)]
pub struct MTGenSubSector<'a> {
    // Execution stage
    stage: GenerationStage,
    // Tile within stage
    tiles: Grid<MTGenTile<'a>>,
}

impl<'a> MTGenSubSector<'a> {
    pub fn new(stage: GenerationStage, tiles: Grid<MTGenTile<'a>>) -> Self {
        Self { stage, tiles }
    }

    pub fn tiles(&self) -> &Grid<MTGenTile<'a>> {
        &self.tiles
    }

    pub fn generation_stage(&self) -> &GenerationStage {
        &self.generation_stage()
    }
}

// The largest sectors can be done individually, all at once. Then we have to get progressively
// smaller in our chunk sizes to allow blending finer points. See https://imgur.com/a/Vts2yFe for a
// diagram —  red sectors are done first, blue sectors second, and green sectors third.
#[derive(Clone, Debug)]
pub enum GenerationStage {
    Primary,
    Secondary,
    Tertiary,
}

fn empty_tiles(grid: &Grid<TerrainGenerationTile>) -> u32 {
    (grid.width() * grid.height()) - grid.tiles().filter(|t| t.contents().tile_set()).count() as u32
}

fn generation_grid_to_sector(name: String, generation_grid: Grid<TerrainGenerationTile>) -> Sector {
    let mut tiles = Grid::new(generation_grid.size().clone());
    generation_grid
        .tiles()
        .filter_map(|item| {
            item.contents().static_tile().as_ref().map(|static_tile| {
                GridItem::new(item.pos().clone(), Tile::new(static_tile.pos().clone(), 1.))
            })
        })
        .for_each(|item| tiles.push(item));
    Sector::new(name, tiles, Vec::new())
}

fn calculate_tile(
    grid: &mut Grid<TerrainGenerationTile>,
    pos: Vector2<u32>,
    static_tiles: &[StaticTileInfo],
) {
    // Find possible adjacent tiles
    // 1. Filter static tiles with regard to edges
    // 2. `choose_tile()` from the filtered list
    let filtered_static_tiles = static_tiles
        .iter()
        .filter(|stat| position_allows_static_tile(&pos, stat, grid))
        .collect::<Vec<_>>();
    if let Some(tile) = grid.tile_mut(pos) {
        // HACK: Improve `choose_tile()` function —  this is temporary
        if let Some(chosen) = filtered_static_tiles.choose(&mut rand::thread_rng()) {
            // Set tile
            // HACK: Remove this (removing the above hack will fix this)
            tile.contents_mut().set_tile(chosen.clone().clone());
            // Remove entropy
            tile.contents_mut().remove_entropy();
        }
    }
}

fn calculate_entropy(
    grid: &mut Grid<TerrainGenerationTile>,
    pos: &Vector2<u32>,
    static_tiles: &[StaticTileInfo],
) {
    let entropy = static_tiles
        .iter()
        .filter(|stat| position_allows_static_tile(pos, stat, grid))
        .count();
    if let Some(tile) = grid.tile_mut(*pos) {
        let contents = tile.contents_mut();
        if !contents.tile_set() {
            tile.contents_mut().set_entropy(entropy as i32);
        }
    }
}

fn calculate_adj_entropies(
    grid: &mut Grid<TerrainGenerationTile>,
    pos: Vector2<u32>,
    static_tiles: &[StaticTileInfo],
) {
    let positions = grid
        .adjacent(pos)
        .map(|t| t.pos().clone())
        .collect::<Vec<_>>();
    positions
        .iter()
        .for_each(|pos| calculate_entropy(grid, pos, static_tiles));
}

fn min_entropy(grid: &Grid<TerrainGenerationTile>) -> Option<Vector2<u32>> {
    if let Some(min_entropy_tile) = grid.tiles().reduce(|acc, t| {
        if let Some(acc_entropy) = acc.contents().entropy() {
            if let Some(t_entropy) = t.contents().entropy() {
                if t_entropy < acc_entropy {
                    t
                } else {
                    acc
                }
            } else {
                acc
            }
        } else {
            t
        }
    }) {
        return Some(min_entropy_tile.pos());
    } else {
        None
    }
}

fn position_allows_static_tile(
    pos: &Vector2<u32>,
    stat: &StaticTileInfo,
    grid: &Grid<TerrainGenerationTile>,
) -> bool {
    // Filtering for edges which *don't* allow this tile. So if this has a length that
    // isn't 0, we know that we can't use this tile

    grid.adjacent(*pos)
        .map(|adj_tile| {
            match Vector2::new(
                adj_tile.pos().x as i32 - pos.x as i32,
                adj_tile.pos().y as i32 - pos.y as i32,
            ) {
                directions::DOWN => static_tile(adj_tile)
                    .as_ref()
                    .map_or_else(|| true, |t| stat.down == t.up),
                directions::LEFT => static_tile(adj_tile)
                    .as_ref()
                    .map_or_else(|| true, |t| stat.left == t.right),
                directions::RIGHT => static_tile(adj_tile)
                    .as_ref()
                    .map_or_else(|| true, |t| stat.right == t.left),
                directions::UP => static_tile(adj_tile)
                    .as_ref()
                    .map_or_else(|| true, |t| stat.up == t.down),
                _ => {
                    false // Is *not* allowed
                }
            }
        })
        .filter(|b| !b) // We're checking for non-matching edges
        .count()
        .eq(&0) // All of the edges match
}

fn static_tile(item: &GridItem<TerrainGenerationTile>) -> &Option<StaticTileInfo> {
    return item.contents().static_tile();
}

// HACK: This is temporary, until I write a proper asset loading system
fn load_tilemap_json() -> Vec<StaticTileInfo> {
    let json = include_str!("../assets/tilemap.json");
    let static_tiles: Vec<StaticTileInfo> = serde_json::from_str(json).unwrap();
    static_tiles
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StaticTileInfo {
    x: u32,
    y: u32,
    down: String,
    left: String,
    up: String,
    right: String,
}

impl StaticTileInfo {
    fn pos(&self) -> Vector2<u32> {
        Vector2::new(self.x, self.y)
    }
}

#[derive(Clone, Debug)]
pub struct TerrainGenerationTile {
    static_tile: Option<StaticTileInfo>,
    entropy: Option<i32>,
}

impl TerrainGenerationTile {
    fn new() -> Self {
        Self {
            static_tile: None,
            entropy: None,
        }
    }

    pub fn static_tile(&self) -> &Option<StaticTileInfo> {
        &self.static_tile
    }

    fn set_tile(&mut self, tile: StaticTileInfo) {
        self.static_tile = Some(tile);
    }

    fn set_entropy(&mut self, entropy: i32) {
        self.entropy = Some(entropy);
    }

    fn remove_entropy(&mut self) {
        self.entropy = None;
    }

    fn tile_set(&self) -> bool {
        self.static_tile.is_some()
    }

    pub fn entropy(&self) -> Option<i32> {
        self.entropy
    }
}
