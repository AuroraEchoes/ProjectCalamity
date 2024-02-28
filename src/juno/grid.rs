use std::{
    default,
    slice::{Iter, IterMut},
};

use cgmath::Vector2;

use super::directions;

#[derive(Clone, Debug)]
pub struct Grid<T> {
    size: Vector2<u32>,
    grid: Vec<GridItem<T>>,
}

impl<T: Clone> Grid<T> {
    pub fn new(size: Vector2<u32>) -> Self {
        let grid = Vec::with_capacity((size.x * size.y) as usize);
        return Self { size, grid };
    }

    pub fn fill(&mut self, item: T) {
        for y in 0..self.height() {
            for x in 0..self.width() {
                self.push(GridItem::new(Vector2::new(x, y), item.clone()));
            }
        }
    }

    pub fn push(&mut self, tile: GridItem<T>) {
        self.grid.push(tile);
    }

    pub fn width(&self) -> u32 {
        self.size.x
    }

    pub fn height(&self) -> u32 {
        self.size.y
    }

    pub fn size(&self) -> Vector2<u32> {
        self.size
    }

    pub fn tile(&self, pos: Vector2<u32>) -> Option<&GridItem<T>> {
        self.grid.get((pos.y * self.width() + pos.x) as usize)
    }

    pub fn tile_mut(&mut self, pos: Vector2<u32>) -> Option<&mut GridItem<T>> {
        self.grid.get_mut((pos.y * self.size.x + pos.x) as usize)
    }

    pub fn tiles(&self) -> Iter<'_, GridItem<T>> {
        return self.grid.iter();
    }

    pub fn tiles_mut(&mut self) -> IterMut<'_, GridItem<T>> {
        return self.grid.iter_mut();
    }

    pub fn adjacent(&self, pos: Vector2<u32>) -> impl Iterator<Item = &GridItem<T>> {
        let adjacent_positions: [Vector2<i32>; 4] = [
            Vector2::new(
                directions::DOWN.x + pos.x as i32,
                directions::DOWN.y + pos.y as i32,
            ),
            Vector2::new(
                directions::LEFT.x + pos.x as i32,
                directions::LEFT.y + pos.y as i32,
            ),
            Vector2::new(
                directions::RIGHT.x + pos.x as i32,
                directions::RIGHT.y + pos.y as i32,
            ),
            Vector2::new(
                directions::UP.x + pos.x as i32,
                directions::UP.y + pos.y as i32,
            ),
        ];
        return self.tiles().filter(move |t| {
            adjacent_positions.contains(&Vector2::new(t.pos().x as i32, t.pos().y as i32))
        });
    }

    pub fn adjacent_mut(&mut self, pos: Vector2<u32>) -> impl Iterator<Item = &mut GridItem<T>> {
        let adjacent_positions: [Vector2<i32>; 4] = [
            Vector2::new(
                directions::DOWN.x + pos.x as i32,
                directions::DOWN.y + pos.y as i32,
            ),
            Vector2::new(
                directions::LEFT.x + pos.x as i32,
                directions::LEFT.y + pos.y as i32,
            ),
            Vector2::new(
                directions::RIGHT.x + pos.x as i32,
                directions::RIGHT.y + pos.y as i32,
            ),
            Vector2::new(
                directions::UP.x + pos.x as i32,
                directions::UP.y + pos.y as i32,
            ),
        ];

        return self.tiles_mut().filter(move |t| {
            adjacent_positions.contains(&Vector2::new(t.pos().x as i32, t.pos().y as i32))
        });
    }

    pub fn adjacent_diagonal(&self, pos: Vector2<u32>) -> impl Iterator<Item = &GridItem<T>> {
        let adjacent_positions: [Vector2<i32>; 8] = [
            Vector2::new(
                directions::DOWN.x + pos.x as i32,
                directions::DOWN.y + pos.y as i32,
            ),
            Vector2::new(
                directions::DOWN_LEFT.x + pos.x as i32,
                directions::DOWN_LEFT.y + pos.y as i32,
            ),
            Vector2::new(
                directions::DOWN_RIGHT.x + pos.x as i32,
                directions::DOWN_RIGHT.y + pos.y as i32,
            ),
            Vector2::new(
                directions::LEFT.x + pos.x as i32,
                directions::LEFT.y + pos.y as i32,
            ),
            Vector2::new(
                directions::RIGHT.x + pos.x as i32,
                directions::RIGHT.y + pos.y as i32,
            ),
            Vector2::new(
                directions::UP.x + pos.x as i32,
                directions::UP.y + pos.y as i32,
            ),
            Vector2::new(
                directions::UP_LEFT.x + pos.x as i32,
                directions::UP_LEFT.y + pos.y as i32,
            ),
            Vector2::new(
                directions::UP_RIGHT.x + pos.x as i32,
                directions::UP_RIGHT.y + pos.y as i32,
            ),
        ];

        return self.tiles().filter(move |t| {
            adjacent_positions.contains(&Vector2::new(t.pos().x as i32, t.pos().y as i32))
        });
    }

    pub fn adjacent_diagonal_mut(
        &mut self,
        pos: Vector2<u32>,
    ) -> impl Iterator<Item = &mut GridItem<T>> {
        let adjacent_positions: [Vector2<i32>; 8] = [
            Vector2::new(
                directions::DOWN.x + pos.x as i32,
                directions::DOWN.y + pos.y as i32,
            ),
            Vector2::new(
                directions::DOWN_LEFT.x + pos.x as i32,
                directions::DOWN_LEFT.y + pos.y as i32,
            ),
            Vector2::new(
                directions::DOWN_RIGHT.x + pos.x as i32,
                directions::DOWN_RIGHT.y + pos.y as i32,
            ),
            Vector2::new(
                directions::LEFT.x + pos.x as i32,
                directions::LEFT.y + pos.y as i32,
            ),
            Vector2::new(
                directions::RIGHT.x + pos.x as i32,
                directions::RIGHT.y + pos.y as i32,
            ),
            Vector2::new(
                directions::UP.x + pos.x as i32,
                directions::UP.y + pos.y as i32,
            ),
            Vector2::new(
                directions::UP_LEFT.x + pos.x as i32,
                directions::UP_LEFT.y + pos.y as i32,
            ),
            Vector2::new(
                directions::UP_RIGHT.x + pos.x as i32,
                directions::UP_RIGHT.y + pos.y as i32,
            ),
        ];

        return self.tiles_mut().filter(move |t| {
            adjacent_positions.contains(&Vector2::new(t.pos().x as i32, t.pos().y as i32))
        });
    }
}

#[derive(Clone, Debug)]
pub struct GridItem<T> {
    pos: Vector2<u32>,
    contents: T,
}

impl<T> GridItem<T> {
    pub fn new(pos: Vector2<u32>, contents: T) -> GridItem<T> {
        return Self { pos, contents };
    }
    pub fn contents(&self) -> &T {
        return &self.contents;
    }

    pub fn contents_mut(&mut self) -> &mut T {
        return &mut self.contents;
    }

    pub fn pos(&self) -> Vector2<u32> {
        return self.pos;
    }
}
