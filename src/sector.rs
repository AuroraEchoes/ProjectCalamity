use std::slice::Iter;

use rand::Rng;
use raylib::color::{Color, self};

use crate::utility::USizeVec2;

pub struct Sector {
    name: String,
    size: USizeVec2,
    tiles: Vec<Color>,
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
    
    pub fn tile<'a>(&'a self, x: usize, y: usize) -> Option<&'a Color> {
        return self.tiles.get(y * self.width() + x);
    }

    pub fn tile_mut<'a>(&'a mut self, x: usize, y: usize) -> Option<&'a mut Color> {
        return self.tiles.get_mut(y * self.size.x() + x);
    }

    pub fn random(name: &str, width: usize, height: usize) -> Self {
        let mut rand = rand::thread_rng();
        let mut tiles = Vec::with_capacity(width * height);
        for _ in 0..(width * height) {
            tiles.push(Color::color_from_hsv(rand.gen(), rand.gen(), rand.gen()));
        }
        return Self { name: name.to_string(), size: USizeVec2::new(width, height), tiles, units: Vec::new() };
    }

    pub fn add_unit(&mut self, u: Unit) {
        self.units.push(u);
    }
}

pub struct Unit {
    pos: USizeVec2,
    color: Color,
}

impl Unit {
    pub fn new(pos: USizeVec2, color: Color) -> Unit {
        return Unit { pos, color };
    }

    pub fn pos(&self) -> &USizeVec2 {
        return &self.pos;
    }

    pub fn color(&self) -> &Color {
        return &self.color;
    }
}