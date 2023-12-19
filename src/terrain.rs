use juno::{
    grid::{Grid, GridItem},
    ivec,
    vector::{IVec2, Vector},
};
use rand::{rngs::ThreadRng, Rng};

use crate::sector::{Sector, Tile};

pub fn generate_terrain(name: String, size: IVec2) -> Sector {
    let (size_one, size_two) = (size.clone(), size.clone());
    let mut grid = Grid::new(size);
    for y in 0..*size_one.y() {
        for x in 0..*size_two.x() {
            grid.push(GridItem::new(
                ivec!(x, y),
                Tile::new(TileType::GrassVar1, 2.),
            ));
        }
    }
    let sector = Sector::new("Testing Sector".to_string(), grid, Vec::new());
    return sector;
}

fn rand_adj(mut rand: &mut ThreadRng, possible: &Vec<TileAdj>) -> TileAdj {
    let index = rand.gen_range(0..possible.len());
    return possible[index].clone();
}

fn calc_entropies(
    index: usize,
    tiles: &mut Vec<Option<TileAdj>>,
    entropies: &mut Vec<Option<u32>>,
    poss_adj: &Vec<TileAdj>,
    width: usize,
) {
    let tile = tiles[index].clone().unwrap();
    // Down
    if index + width < entropies.len() {
        entropies[index + width] =
            Some(poss_adj.iter().filter(|t| t.up == tile.down).count() as u32);
    }
    // Left
    if index as i32 - 1 > 0 {
        entropies[index - 1] =
            Some(poss_adj.iter().filter(|t| t.right == tile.left).count() as u32);
    }
    // Right
    if index + 1 < entropies.len() {
        entropies[index + 1] =
            Some(poss_adj.iter().filter(|t| t.left == tile.right).count() as u32);
    }
    // Up
    if index as i32 - width as i32 > 0 {
        entropies[index - width] =
            Some(poss_adj.iter().filter(|t| t.down == tile.up).count() as u32);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TileAdj {
    pos: IVec2,
    down: Edge,
    left: Edge,
    right: Edge,
    up: Edge,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Edge {
    Grass,
    Path,
}

#[derive(PartialEq)]
pub enum TileType {
    GrassNone,
    GrassVar1,
    GrassVar2,
    PathBottom,
    PathBottomRight,
    PathBottomLeftRight,
    PathTop,
}
