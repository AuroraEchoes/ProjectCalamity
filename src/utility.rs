use std::{cmp::Ordering, ops::{Div, Mul}};

use raylib::math::Vector2;

#[derive(Clone, Copy, Debug)]
pub struct USizeVec2 {
    x: usize,
    y: usize,
}

impl USizeVec2 {
    pub fn new(x: usize, y: usize) -> USizeVec2 {
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
        self.x -= n;
    }

    pub fn add_y(&mut self, n: usize) {
        self.y += n;
    }

    pub fn subtract_y(&mut self, n: usize) {
        self.y -= n;
    }


}

impl PartialEq for USizeVec2 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl PartialOrd for USizeVec2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.x.cmp(&other.x), self.y.cmp(&other.y)) {
            (Ordering::Greater, Ordering::Greater) => Some(Ordering::Greater),
            (Ordering::Less, Ordering::Less) => Some(Ordering::Less),
            _ => Some(Ordering::Equal),
        }
    }
}

impl Mul for USizeVec2 {
    type Output = USizeVec2;

    fn mul(mut self, rhs: Self) -> Self::Output {
        self.x *= rhs.x;
        self.y *= rhs.y;
        return self;
    }
}

impl Mul<usize> for USizeVec2 {
    type Output = USizeVec2;

    fn mul(mut self, rhs: usize) -> Self::Output {
        self.x *= rhs;
        self.y *= rhs;
        return self;
    }
}

impl Div for USizeVec2 {
    type Output = USizeVec2;

    fn div(self, rhs: Self) -> Self::Output {
        return USizeVec2::new(self.x / rhs.x, self.y / rhs.y);
    }
}

impl Div<usize> for USizeVec2 {
    type Output = USizeVec2;

    fn div(mut self, rhs: usize) -> Self::Output {
        self.x /= rhs;
        self.y /= rhs;
        return self;
    }
}

impl From<USizeVec2> for Vector2 {
    fn from(value: USizeVec2) -> Self {
        return Vector2 { x: value.x as f32, y: value.y as f32 };
    }
}