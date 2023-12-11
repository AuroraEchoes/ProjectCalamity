use rand::{rngs::ThreadRng, Rng};

use crate::{
    sector::{Sector, Tile},
    utility::GridPosVec,
};

pub fn generate_terrain(name: String, size: GridPosVec) -> Sector {
    let mut random = rand::thread_rng();
    let starting_tile = random.gen_range(0..(size.x() * size.y()));
    let mut tiles = vec![None; size.x() * size.y()];
    let mut entropies = vec![None; size.x() * size.y()];
    let possible_tiles = vec![
        TileAdj {
            pos: GridPosVec::new(0, 0),
            down: Edge::Grass,
            left: Edge::Grass,
            right: Edge::Grass,
            up: Edge::Grass,
        },
        TileAdj {
            pos: GridPosVec::new(1, 0),
            down: Edge::Grass,
            left: Edge::Grass,
            right: Edge::Grass,
            up: Edge::Grass,
        },
        TileAdj {
            pos: GridPosVec::new(2, 0),
            down: Edge::Grass,
            left: Edge::Grass,
            right: Edge::Grass,
            up: Edge::Grass,
        },
        TileAdj {
            pos: GridPosVec::new(3, 0),
            down: Edge::Path,
            left: Edge::Grass,
            right: Edge::Grass,
            up: Edge::Grass,
        },
        TileAdj {
            pos: GridPosVec::new(4, 0),
            down: Edge::Path,
            left: Edge::Grass,
            right: Edge::Path,
            up: Edge::Grass,
        },
        TileAdj {
            pos: GridPosVec::new(5, 0),
            down: Edge::Path,
            left: Edge::Path,
            right: Edge::Path,
            up: Edge::Grass,
        },
        TileAdj {
            pos: GridPosVec::new(3, 2),
            down: Edge::Grass,
            left: Edge::Grass,
            right: Edge::Grass,
            up: Edge::Path,
        },
    ];

    tiles[starting_tile] = Some(rand_adj(&mut random, &possible_tiles));
    calc_entropies(
        starting_tile,
        &mut tiles,
        &mut entropies,
        &possible_tiles,
        size.x(),
    );
    let mut iter_count = 0;
    while tiles.contains(&None) {
        iter_count += 1;
        if iter_count >= 100 {
            for t in tiles.iter_mut() {
                *t = Some(TileAdj {
                    pos: GridPosVec::new(0, 0),
                    down: Edge::Grass,
                    left: Edge::Grass,
                    right: Edge::Grass,
                    up: Edge::Grass,
                });
            }
        }
        // Choose the smallest unset tile by entropy
        let mut smallest = None;
        let sorted_entropies = entropies
            .iter()
            .filter_map(|x| *x)
            .enumerate()
            .collect::<Vec<_>>();
        for (i, ent) in sorted_entropies.iter() {
            println!("boop {sorted_entropies:?}");
            if let Some((s_i, s_ent)) = smallest {
                if s_ent < ent {
                    if let None = tiles[i.clone()] {
                        smallest = Some((i, ent));
                    }
                }
            } else {
                smallest = Some((i, ent));
            }
        }
        let s_index = smallest.unwrap().0;
        let down = tiles.get((s_index.clone() as i32 + size.x() as i32) as usize);
        let left = tiles.get((s_index.clone() as i32 - 1) as usize);
        let right = tiles.get((s_index.clone() as i32 + 1) as usize);
        let up = tiles.get((s_index.clone() as i32 - size.x() as i32) as usize);
        let this_pos_tiles = possible_tiles
            .iter()
            .filter(|t| {
                if let Some(Some(d)) = down {
                    d.down == t.up
                } else {
                    true
                }
            })
            .filter(|t| {
                if let Some(Some(l)) = left {
                    l.left == t.right
                } else {
                    true
                }
            })
            .filter(|t| {
                if let Some(Some(r)) = right {
                    r.right == t.left
                } else {
                    true
                }
            })
            .filter(|t| {
                if let Some(Some(u)) = up {
                    u.up == t.down
                } else {
                    true
                }
            })
            .map(|t| t.clone())
            .collect::<Vec<_>>();
        let tile;
        if this_pos_tiles.len() != 0 {
            tile = rand_adj(&mut random, &this_pos_tiles);
        } else {
            println!("nope");
            tile = possible_tiles[0].clone();
        }
        tiles[s_index.clone()] = Some(tile.clone());
        calc_entropies(
            s_index.clone(),
            &mut tiles,
            &mut entropies,
            &possible_tiles,
            size.x(),
        );
    }
    let mut tiles_converted = Vec::new();
    for (i, tile) in tiles.iter().enumerate() {
        let tile = tile.clone().unwrap();
        let tile_type = match tile.pos.x() + tile.pos.y() * 10 {
            0 => TileType::GrassNone,
            1 => TileType::GrassVar1,
            2 => TileType::GrassVar2,
            3 => TileType::PathBottom,
            4 => TileType::PathBottomRight,
            5 => TileType::PathBottomLeftRight,
            23 => TileType::PathTop,
            _ => TileType::GrassNone,
        };
        tiles_converted.push(Tile::new(
            GridPosVec::new(i % size.x(), i / size.x()),
            tile_type,
            1.,
        ))
    }
    let sector = Sector::new(name, size, tiles_converted, Vec::new());
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
    pos: GridPosVec,
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
