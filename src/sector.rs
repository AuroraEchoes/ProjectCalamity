use std::slice::Iter;

use rand::Rng;
use raylib::color::{Color, self};

use crate::utility::USizeVec2;

pub struct Sector {
    name: String,
    size: USizeVec2,
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

    pub fn size(&self) -> &USizeVec2 {
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

    pub fn tile_mut(&mut self, x: usize, y: usize) -> Option<&mut Tile> {
        return self.tiles.get_mut(y * self.size.x() + x);
    }

    pub fn random(name: &str, width: usize, height: usize) -> Self {
        let mut rand = rand::thread_rng();
        let mut tiles = Vec::with_capacity(width * height);
        for _ in 0..(width * height) {
            tiles.push(Tile {
                color: Color::color_from_hsv(rand.gen(), rand.gen(), rand.gen()),
                speed_modifier: 1.,
            });
        }
        return Self { name: name.to_string(), size: USizeVec2::new(width, height), tiles, units: Vec::new() };
    }

    pub fn add_unit(&mut self, u: Unit) {
        self.units.push(u);
    }

    pub fn unit_at_tile(&self, pos: USizeVec2) -> Option<&Unit> {
        return self.units.iter().filter(|u| u.pos == pos).nth(0);
    }
}

pub struct Unit {
    pos: USizeVec2,
    color: Color,
    movement: f32,
}

impl Unit {
    pub fn new(pos: USizeVec2, color: Color, movement: f32) -> Unit {
        return Unit { pos, color, movement };
    }

    pub fn pos(&self) -> &USizeVec2 {
        return &self.pos;
    }

    pub fn color(&self) -> &Color {
        return &self.color;
    }
}

pub struct Tile {
    color: Color,
    speed_modifier: f32,
}

impl Tile {
    pub fn new(color: Color, speed_modifier: f32) -> Self {
        return Self { color, speed_modifier }
    }

    pub fn speed_modifier(&self) -> f32 {
        return self.speed_modifier;
    }

    pub fn color(&self) -> Color {
        return self.color;
    }
}

pub struct NavigationDistanceField {
    size: USizeVec2,
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
        
    }
}