use std::{cmp::Ordering, ops::{Div, Mul}};

use log::warn;
use raylib::math::Vector2;

#[derive(Clone, Copy, Debug)]
pub struct GridPosVec {
    x: usize,
    y: usize,
}

impl GridPosVec {
    pub fn new(x: usize, y: usize) -> GridPosVec {
        return Self { x, y };
    }

    pub fn x(&self) -> usize {
        return self.x;
    }

    pub fn y(&self) -> usize {
        return self.y;
    }

    pub fn add_x(&mut self, n: usize) {
        self.x += n;
    }

    pub fn subtract_x(&mut self, n: usize) {
        if n >= self.x {
            self.x -= n;
        } else {
            warn!("Attempted to underflow sector position");
        }
    }

    pub fn add_y(&mut self, n: usize) {
        self.y += n;
    }

    pub fn subtract_y(&mut self, n: usize) {
        if n >= self.y {
            self.y -= n;
        } else {
            warn!("Attempted to underflow sector position");
        }
    }


}

impl PartialEq for GridPosVec {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl PartialOrd for GridPosVec {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.x.cmp(&other.x), self.y.cmp(&other.y)) {
            (Ordering::Greater, Ordering::Greater) => Some(Ordering::Greater),
            (Ordering::Less, Ordering::Less) => Some(Ordering::Less),
            _ => Some(Ordering::Equal),
        }
    }
}

impl Mul for GridPosVec {
    type Output = GridPosVec;

    fn mul(mut self, rhs: Self) -> Self::Output {
        self.x *= rhs.x;
        self.y *= rhs.y;
        return self;
    }
}

impl Mul<usize> for GridPosVec {
    type Output = GridPosVec;

    fn mul(mut self, rhs: usize) -> Self::Output {
        self.x *= rhs;
        self.y *= rhs;
        return self;
    }
}

impl Div for GridPosVec {
    type Output = GridPosVec;

    fn div(self, rhs: Self) -> Self::Output {
        return GridPosVec::new(self.x / rhs.x, self.y / rhs.y);
    }
}

impl Div<usize> for GridPosVec {
    type Output = GridPosVec;

    fn div(mut self, rhs: usize) -> Self::Output {
        self.x /= rhs;
        self.y /= rhs;
        return self;
    }
}

impl From<GridPosVec> for Vector2 {
    fn from(value: GridPosVec) -> Self {
        return Vector2 { x: value.x as f32, y: value.y as f32 };
    }
}