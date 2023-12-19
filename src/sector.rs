use std::slice::{Iter, IterMut};

use anyhow::Context;
use juno::{
    grid::{Grid, GridItem},
    ivec,
    vector::{IVec2, Vector},
};
use raylib::color::Color;

use crate::terrain::TileType;

pub struct Sector {
    name: String,
    tiles: Grid<Tile>,
    units: Vec<Unit>,
}

impl Sector {
    pub fn new(name: String, tiles: Grid<Tile>, units: Vec<Unit>) -> Self {
        return Self { name, tiles, units };
    }

    pub fn width(&self) -> &i32 {
        return self.tiles.size().x();
    }

    pub fn height(&self) -> &i32 {
        return self.tiles.size().y();
    }

    pub fn size(&self) -> &IVec2 {
        return &self.tiles.size();
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

    pub fn tile(&self, pos: &IVec2) -> Option<&GridItem<Tile>> {
        return self.tiles.tile(pos);
    }

    pub fn tile_mut(&mut self, pos: &IVec2) -> Option<&mut GridItem<Tile>> {
        return self.tiles.tile_mut(pos);
    }

    pub fn tiles(&self) -> Iter<'_, GridItem<Tile>> {
        return self.tiles.tiles();
    }

    pub fn add_unit(&mut self, u: Unit) {
        self.units.push(u);
    }

    pub fn unit_at_tile(&self, pos: IVec2) -> Option<&Unit> {
        return self.units.iter().filter(|u| u.pos == pos).nth(0);
    }

    pub fn unit_at_tile_mut(&mut self, pos: IVec2) -> Option<&mut Unit> {
        return self.units.iter_mut().filter(|u| u.pos == pos).nth(0);
    }

    pub fn index(&self, pos: &IVec2) -> i32 {
        return self.size().x() * pos.y() + pos.x();
    }

    pub fn from_index(&self, index: i32) -> IVec2 {
        return ivec!(index % self.width(), index / self.width());
    }
}

pub struct Unit {
    pos: IVec2,
    nav: Option<NavigationBitmask>,
    color: Color,
    movement: f32,
}

impl Unit {
    pub fn new(pos: IVec2, color: Color, movement: f32, sector: &Sector) -> Result<Unit, ()> {
        let mut unit = Unit {
            pos,
            nav: None,
            color,
            movement,
        };
        unit.nav = Some(NavigationBitmask::generate(&unit, sector)?);
        return Ok(unit);
    }

    pub fn pos(&self) -> &IVec2 {
        return &self.pos;
    }

    pub fn color(&self) -> &Color {
        return &self.color;
    }

    pub fn movement(&self) -> &f32 {
        return &self.movement;
    }

    pub fn can_reach_tile(&self, pos: &IVec2, sector: &Sector) -> Option<&bool> {
        if let Some(nav) = &self.nav {
            return nav.tile(pos, sector);
        }
        return None;
    }
}

pub struct NavigationBitmask {
    movable_tiles: Vec<bool>,
    size: IVec2,
}

impl NavigationBitmask {
    fn generate(unit: &Unit, sector: &Sector) -> Result<Self, ()> {
        let mut tile_costs = vec![None::<f32>; (sector.width() * sector.height()) as usize];
        tile_costs[sector.index(unit.pos()) as usize] = Some(0.);
        let sector_unx = sector.width() - unit.pos().x();
        let sector_uny = sector.height() - unit.pos().y();
        let max_radius = *[unit.pos().x(), &sector_unx, unit.pos().y(), &sector_uny]
            .iter()
            .min()
            .context("Could not get min radius")
            .unwrap();
        for r in 0..*max_radius {
            let x_low = unit.pos().x() - r;
            let x_high = unit.pos().x() + r;
            let y_low = unit.pos().y() - r;
            let y_high = unit.pos().y() + r;
            let mut tiles_to_replace = Vec::<(usize, f32)>::with_capacity(8);
            tile_costs
                .iter()
                .enumerate()
                .filter(|(i, _)| {
                    let pos = sector.from_index(*i as i32);
                    return pos.x() == &x_low
                        || pos.x() == &x_high
                        || pos.y() == &y_low
                        || pos.y() == &y_high;
                })
                .for_each(|(i, cost_opt)| {
                    let pos = sector.from_index(i as i32);
                    if let Some(cost) = cost_opt {
                        let adj = NavigationBitmask::adjacent_mut(&tile_costs, &pos, sector);
                        for (a, p) in adj {
                            if let Some(a_tile) = sector.tile(&p) {
                                let cost_for_tile = cost + 1. / a_tile.contents().speed_modifier;
                                match a {
                                    Some(prev_cost) => {
                                        if prev_cost > &cost_for_tile {
                                            tiles_to_replace
                                                .push((sector.index(&p) as usize, cost_for_tile));
                                        }
                                    }
                                    None => tiles_to_replace
                                        .push((sector.index(&p) as usize, cost_for_tile)),
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
        pos: &IVec2,
        sector: &Sector,
    ) -> Vec<(&'a Option<f32>, IVec2)> {
        let offsets = [[0, 1], [1, 0], [0, -1], [-1, 0]];
        let indexes = offsets
            .iter()
            .map(|x| {
                let mut pos = pos.clone();
                pos.transform(&ivec!(x[0], x[1]));
                return sector.index(&pos);
            })
            .collect::<Vec<_>>();
        return tiles
            .iter()
            .enumerate()
            .filter(|(i, _)| indexes.contains(&(*i as i32)))
            .map(|(i, x)| (x, sector.from_index(i as i32)))
            .collect::<Vec<_>>();
    }

    fn size(&self) -> &IVec2 {
        return &self.size;
    }

    fn tile(&self, pos: &IVec2, sector: &Sector) -> Option<&bool> {
        return self.movable_tiles.get(sector.index(pos) as usize);
    }
}

pub struct Tile {
    tile_type: TileType,
    speed_modifier: f32,
}

impl Tile {
    pub fn new(tile_type: TileType, speed_modifier: f32) -> Self {
        return Self {
            tile_type,
            speed_modifier,
        };
    }

    pub fn speed_modifier(&self) -> f32 {
        return self.speed_modifier;
    }

    pub fn tile_type(&self) -> &TileType {
        return &self.tile_type;
    }
}
