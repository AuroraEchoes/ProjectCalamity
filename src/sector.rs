use std::slice::Iter;

use rand::Rng;
use raylib::color::{Color, self};

use crate::utility::GridPosVec;

pub struct Sector {
    name: String,
    size: GridPosVec,
    tiles: Vec<Tile>,
    units: Vec<Unit>
}

impl Sector {
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
    
    pub fn tile(&self, x: usize, y: usize) -> Option<&Tile> {
        return self.tiles.get(y * self.width() + x);
    }

    pub fn tile_pos(&self, pos: GridPosVec) -> Option<&Tile> {
        return self.tile(pos.x(), pos.y());
    }

    pub fn tile_mut(&mut self, x: usize, y: usize) -> Option<&mut Tile> {
        return self.tiles.get_mut(y * self.size.x() + x);
    }

    pub fn random(name: &str, width: usize, height: usize) -> Self {
        let mut rand = rand::thread_rng();
        let mut tiles = Vec::with_capacity(width * height);
        for i in 0..(width * height) {
            tiles.push(Tile {
                pos: GridPosVec::new(i % width, i / width),
                color: Color::color_from_hsv(rand.gen(), rand.gen(), rand.gen()),
                speed_modifier: 1.,
            });
        }
        return Self { name: name.to_string(), size: GridPosVec::new(width, height), tiles, units: Vec::new() };
    }

    pub fn add_unit(&mut self, u: Unit) {
        self.units.push(u);
    }

    pub fn unit_at_tile(&self, pos: GridPosVec) -> Option<&Unit> {
        return self.units.iter().filter(|u| u.pos == pos).nth(0);
    }

    pub fn adjacent(&self, subject: &Tile) -> Vec<&Tile> {
        let adjacent: [[i32; 2]; 8] = [[0, 1], [1, 1], [1, 0], [1, -1], [0, -1], [-1, -1], [-1, 0], [-1, 1]];
        let pos = subject.pos;
        let mut adjacent_tiles = Vec::with_capacity(adjacent.len());
        for off in adjacent {
            if let Some(tile) = self.tile((pos.x() as i32 + off[0]) as usize, (pos.y() as i32 + off[1]) as usize) {
                adjacent_tiles.push(tile);
            }
        }
        return adjacent_tiles;
    }
}

pub struct Unit {
    pos: GridPosVec,
    color: Color,
    movement: f32,
}

impl Unit {
    pub fn new(pos: GridPosVec, color: Color, movement: f32) -> Unit {
        return Unit { pos, color, movement };
    }

    pub fn pos(&self) -> &GridPosVec {
        return &self.pos;
    }

    pub fn color(&self) -> &Color {
        return &self.color;
    }
}

pub struct Tile {
    pos: GridPosVec,
    color: Color,
    speed_modifier: f32,
}

impl Tile {
    pub fn new(pos: GridPosVec, color: Color, speed_modifier: f32) -> Self {
        return Self { pos, color, speed_modifier }
    }

    pub fn speed_modifier(&self) -> f32 {
        return self.speed_modifier;
    }

    pub fn pos(&self) -> &GridPosVec {
        return &self.pos;
    }

    pub fn color(&self) -> &Color {
        return &self.color;
    }
}

pub struct NavigationDistanceField {
    size: GridPosVec,
    distances: Vec<Option<f32>>
}

impl NavigationDistanceField {
    fn new(sector: &Sector) -> Self {
        let size = sector.size().clone();
        let mut distances = Vec::with_capacity(size.x() * size.y());
        distances.fill(None);
        return Self { size, distances };
    }

    fn fill(&mut self, unit: &Unit, sector: &Sector) {
        self.distances[self.size.x() * (unit.pos().y() - 1) + unit.pos().x()] = Some(0.);
        sector.tile_pos(*unit.pos());
        
    }

    fn calc_cost(&mut self, tile: &Tile, sector: &Sector) {
        let adjacent = sector.adjacent(tile);

    }
}