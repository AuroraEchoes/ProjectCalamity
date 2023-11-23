use std::{slice::{Iter, IterMut}, cell::RefCell};

use log::warn;
use rand::Rng;
use raylib::color::Color;

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

    pub fn units_mut(&mut self) -> IterMut<'_, Unit> {
        return self.units.iter_mut();
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

    pub fn unit_at_tile_mut(&mut self, pos: GridPosVec) -> Option<&mut Unit> {
        return self.units.iter_mut().filter(|u| u.pos == pos).nth(0);
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

    pub fn generate_navs(&mut self) {
        for x in 0..self.width() {
            for y in 0..self.height() {
                println!("x: {:?}, y: {:?}, nav: {:?}", x, y, self.populate(GridPosVec::new(x, y))); 
            }
        }
    }

    pub fn populate(&self, start: GridPosVec) -> Vec<f32> {
        let edges: [[i32; 2]; 4] = [[1, 0], [0, 1], [-1, 0], [0, -1]];
        let mut compute_queue = vec![start];
        let mut computed = Vec::<(f32, GridPosVec)>::with_capacity(&self.width() * &self.height());
        computed.push((0., start));
        while compute_queue.len() > 0 {
            let parents = compute_queue.clone();
            compute_queue.clear();
            for parent in parents.iter() {
                if let Some((parent_cost, _)) = computed
                    .clone()
                    .iter()
                    .filter(|(_, p_pos)| p_pos == parent)
                    .nth(0)
                {
                    let children = edges
                        .map(|edge| {
                            let edge_x = parent.x() as i32 + edge[0];
                            let edge_y = parent.y() as i32 + edge[1];
                            if edge_x >= 0 && edge_x < self.width() as i32 && edge_y >= 0 && edge_y < self.height() as i32 {
                                return self.tile(edge_x as usize, edge_y as usize);
                            } else {
                                return None;
                            }
                        });
                    for child in children.iter() {
                        if let Some(child) = child {
                            compute_queue.push(child.pos().clone());
                            match computed
                                .iter_mut()
                                .filter(|(_, c_pos)| c_pos == child.pos())
                                .nth(0) 
                            {
                                Some((prev_cost, _)) => {
                                    let new_cost = parent_cost + 1. / child.speed_modifier;
                                    if *prev_cost > new_cost {
                                        *prev_cost = new_cost;
                                    }
                                },
                                None => computed.push((parent_cost + 1. / child.speed_modifier, child.pos().clone())),
                            }
                        }
                    }
                }
            }
        }    

        let mut final_vec = Vec::with_capacity(self.width() * self.height());
        for x in 0..self.width() {
            for y in 0..self.height() {
                if let Some((cost, _)) = computed
                    .iter()
                    .filter(|(_, pos)| pos.x() == x && pos.y() == y)
                    .nth(0) 
                {
                    final_vec.push(*cost);
                } else {
                    warn!("Could not find computed cost value for ({:?}, {:?}) at start ({:?}, {:?})", x, y, start.x(), start.y());
                }
            }
        }
        return final_vec;
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

    pub fn movement(&self) -> &f32 {
        return &self.movement;
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