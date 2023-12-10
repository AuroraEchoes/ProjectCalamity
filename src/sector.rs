use std::slice::{Iter, IterMut};

use anyhow::Context;
use raylib::color::Color;

use crate::{terrain::TileType, utility::GridPosVec};

pub struct Sector {
    name: String,
    size: GridPosVec,
    tiles: Vec<Tile>,
    units: Vec<Unit>,
}

impl Sector {
    pub fn new(name: String, size: GridPosVec, tiles: Vec<Tile>, units: Vec<Unit>) -> Self {
        return Self {
            name,
            size,
            tiles,
            units,
        };
    }

    pub fn width(&self) -> usize {
        return self.size.x();
    }

    pub fn height(&self) -> usize {
        return self.size.y();
    }

    pub fn size(&self) -> &GridPosVec {
        return &self.size;
    }

    pub fn name(&self) -> &str {
        return self.name.as_str();
    }

    pub fn units(&self) -> Iter<'_, Unit> {
        return self.units.iter();
    }

    pub fn units_mut(&mut self) -> IterMut<'_, Unit> {
        return self.units.iter_mut();
    }

    pub fn tiles(&self) -> Iter<'_, Tile> {
        return self.tiles.iter();
    }

    pub fn tile(&self, x: usize, y: usize) -> Option<&Tile> {
        return self.tiles.get(y * self.width() + x);
    }

    pub fn tile_mut(&mut self, x: usize, y: usize) -> Option<&mut Tile> {
        let w = self.width();
        return self.tiles.get_mut(y * w + x);
    }

    pub fn tile_pos(&self, pos: GridPosVec) -> Option<&Tile> {
        return self.tile(pos.x(), pos.y());
    }

    pub fn add_unit(&mut self, u: Unit) {
        self.units.push(u);
    }

    pub fn unit_at_tile(&self, pos: GridPosVec) -> Option<&Unit> {
        return self.units.iter().filter(|u| u.pos == pos).nth(0);
    }

    pub fn unit_at_tile_mut(&mut self, pos: GridPosVec) -> Option<&mut Unit> {
        return self.units.iter_mut().filter(|u| u.pos == pos).nth(0);
    }
}

pub struct Unit {
    pos: GridPosVec,
    nav: Option<NavigationBitmask>,
    color: Color,
    movement: f32,
}

impl Unit {
    pub fn new(pos: GridPosVec, color: Color, movement: f32, sector: &Sector) -> Result<Unit, ()> {
        let mut unit = Unit {
            pos,
            nav: None,
            color,
            movement,
        };
        unit.nav = Some(NavigationBitmask::generate(&unit, sector)?);
        return Ok(unit);
    }

    pub fn pos(&self) -> &GridPosVec {
        return &self.pos;
    }

    pub fn color(&self) -> &Color {
        return &self.color;
    }

    pub fn movement(&self) -> &f32 {
        return &self.movement;
    }

    pub fn can_reach_tile(&self, pos: &GridPosVec) -> Option<&bool> {
        if let Some(nav) = &self.nav {
            return nav.tile(pos);
        }
        return None;
    }
}

pub struct NavigationBitmask {
    movable_tiles: Vec<bool>,
    size: GridPosVec,
}

impl NavigationBitmask {
    fn generate(unit: &Unit, sector: &Sector) -> Result<Self, ()> {
        let mut tile_costs = vec![None::<f32>; sector.width() * sector.height()];
        tile_costs[unit.pos().index(sector.size())] = Some(0.);
        // TODO Should this be min?
        let max_radius = *[
            unit.pos().x(),
            sector.width() - unit.pos().x(),
            unit.pos().y(),
            sector.height() - unit.pos().y(),
        ]
        .iter()
        .min()
        .context("Could not get min radius")
        .unwrap();
        for r in 0..max_radius {
            let x_low = unit.pos().x() - r;
            let x_high = unit.pos().x() + r;
            let y_low = unit.pos().y() - r;
            let y_high = unit.pos().y() + r;
            let mut tiles_to_replace = Vec::<(usize, f32)>::with_capacity(8);
            tile_costs
                .iter()
                .enumerate()
                .filter(|(i, _)| {
                    let pos = GridPosVec::from_index(*i, sector.size());
                    return pos.x() == x_low
                        || pos.x() == x_high
                        || pos.y() == y_low
                        || pos.y() == y_high;
                })
                .for_each(|(i, cost_opt)| {
                    let pos = GridPosVec::from_index(i, sector.size());
                    if let Some(cost) = cost_opt {
                        let adj = NavigationBitmask::adjacent_mut(&tile_costs, &pos, sector);
                        for (a, p) in adj {
                            if let Some(a_tile) = sector.tile(p.x(), p.y()) {
                                let cost_for_tile = cost + 1. / a_tile.speed_modifier;
                                match a {
                                    Some(prev_cost) => {
                                        if prev_cost > &cost_for_tile {
                                            tiles_to_replace
                                                .push((p.index(sector.size()), cost_for_tile));
                                        }
                                    }
                                    None => tiles_to_replace
                                        .push((p.index(sector.size()), cost_for_tile)),
                                }
                            }
                        }
                    }
                });
            for (index, cost) in tiles_to_replace {
                *tile_costs
                    .get_mut(index)
                    .context("Could not get tile cost")
                    .unwrap() = Some(cost);
            }
        }

        let movable_tiles = tile_costs
            .iter()
            .map(|x| match x {
                Some(c) => c <= unit.movement(),
                None => false,
            })
            .collect::<Vec<_>>();

        return Ok(Self {
            movable_tiles,
            size: sector.size().clone(),
        });
    }

    fn adjacent_mut<'a>(
        tiles: &'a Vec<Option<f32>>,
        pos: &GridPosVec,
        sector: &Sector,
    ) -> Vec<(&'a Option<f32>, GridPosVec)> {
        let offsets = [[0, 1], [1, 0], [0, -1], [-1, 0]];
        let indexes = offsets
            .iter()
            .map(|x| GridPosVec::index(&pos.offset(x[0], x[1]), sector.size()))
            .collect::<Vec<_>>();
        return tiles
            .iter()
            .enumerate()
            .filter(|(i, _)| indexes.contains(i))
            .map(|(i, x)| (x, GridPosVec::from_index(i, sector.size())))
            .collect::<Vec<_>>();
    }

    fn size(&self) -> &GridPosVec {
        return &self.size;
    }

    fn tile(&self, pos: &GridPosVec) -> Option<&bool> {
        return self.movable_tiles.get(pos.index(self.size()));
    }
}

pub struct Tile {
    pos: GridPosVec,
    tile_type: TileType,
    speed_modifier: f32,
}

impl Tile {
    pub fn new(pos: GridPosVec, tile_type: TileType, speed_modifier: f32) -> Self {
        return Self {
            pos,
            tile_type,
            speed_modifier,
        };
    }

    pub fn speed_modifier(&self) -> f32 {
        return self.speed_modifier;
    }

    pub fn pos(&self) -> &GridPosVec {
        return &self.pos;
    }

    pub fn tile_type(&self) -> &TileType {
        return &self.tile_type;
    }
}
