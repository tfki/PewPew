use std::ops::Add;

pub mod texture;
pub mod movement;

#[derive(Copy, Clone)]
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

pub struct Hitbox {
    pub anchor: Point,
    pub width: u32,
    pub height: u32,
}
