use std::ops::Add;
use crate::gui::engine::components::point_with_alignment::PointWithAlignment;

pub mod hitbox;
pub mod movement;
pub mod point_with_alignment;
pub mod texture;
pub mod timer;

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

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Text {
    pub text: String,
    pub position: PointWithAlignment,

    /// store scale as a fraction, so a text can derive 'Hash' to implement caching
    pub scale_numerator: u32,
    /// store scale as a fraction, so a text can derive 'Hash' to implement caching
    pub scale_denominator: u32,

    /// as a text is movable by a movement component
    /// its original point needs to be stored
    /// as the movement function outputs a point difference to an original position
    pub original_point: Point,

    pub color: sdl2::pixels::Color,
}

impl Text {
    pub fn scale(&self) -> f32 {
        self.scale_numerator as f32 / self.scale_denominator as f32
    }
}
