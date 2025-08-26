use std::ops::Add;
use crate::gui::engine::components::point_with_alignment::PointWithAlignment;

pub mod hitbox;
pub mod movement;
pub mod point_with_alignment;
pub mod texture;
pub mod timer;

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

pub struct Text {
    pub text: String,
    pub position: PointWithAlignment,
    pub scale: f32,
    pub color: sdl2::pixels::Color,
}
