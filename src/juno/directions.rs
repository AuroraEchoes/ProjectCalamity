use std::slice::Iter;

use cgmath::Vector2;

pub const DOWN: Vector2<i32> = Vector2 { x: 0, y: -1 };
pub const DOWN_LEFT: Vector2<i32> = Vector2 { x: -1, y: -1 };
pub const DOWN_RIGHT: Vector2<i32> = Vector2 { x: 1, y: -1 };
pub const LEFT: Vector2<i32> = Vector2 { x: -1, y: 0 };
pub const RIGHT: Vector2<i32> = Vector2 { x: 1, y: 0 };
pub const UP: Vector2<i32> = Vector2 { x: 0, y: 1 };
pub const UP_LEFT: Vector2<i32> = Vector2 { x: -1, y: 1 };
pub const UP_RIGHT: Vector2<i32> = Vector2 { x: 1, y: 1 };

pub fn cardinal() -> Iter<'static, Vector2<i32>> {
    [DOWN, LEFT, RIGHT, UP].iter()
}

pub fn u32_i32_subtract(v1: Vector2<u32>, v2: Vector2<i32>) -> Option<Vector2<u32>> {
    let v1_x = v1.x as i32;
    let v1_y = v1.y as i32;
    if (v1_x + v2.x >= 0) && (v1_y + v2.y >= 0) {
        Some(Vector2::new((v1_x - v2.x) as u32, (v1_y - v2.y) as u32))
    } else {
        None
    }
}

pub fn u32_u32_subtract(v1: Vector2<u32>, v2: Vector2<u32>) -> Vector2<u32> {
    Vector2::new(v1.x - v2.x, v1.y - v2.y)
}

pub fn i32_u32_cast(v: Vector2<i32>) -> Option<Vector2<u32>> {
    if v.x >= 0 && v.y >= 0 {
        Some(Vector2::new(v.x as u32, v.y as u32))
    } else {
        None
    }
}

pub fn u32_i32_cast(v: Vector2<u32>) -> Vector2<i32> {
    Vector2::new(v.x as i32, v.y as i32)
}
