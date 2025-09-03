use std::ops::Add;

pub mod hitbox;
pub mod movement;
pub mod point_with_alignment;
pub mod texture;
pub mod timer;
pub mod text;
pub mod action;

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.x += rhs.x;
        self.y += rhs.y;
        self
    }
}
