use crate::gui::engine::components::point_with_alignment::PointWithAlignment;
use crate::gui::engine::components::Point;

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

pub struct Builder {
    pub text: String,
    pub position: PointWithAlignment,

    /// store scale as a fraction, so a text can derive 'Hash' to implement caching
    pub scale_numerator: u32,
    /// store scale as a fraction, so a text can derive 'Hash' to implement caching
    pub scale_denominator: u32,

    pub color: sdl2::pixels::Color,
}

impl Builder {
    pub fn new(text: String, position: PointWithAlignment) -> Self {
        Builder {
            text,
            position,
            scale_numerator: 1,
            scale_denominator: 1,
            color: sdl2::pixels::Color::WHITE,
        }
    }

    pub fn with_color(mut self, color: sdl2::pixels::Color) -> Self {
        self.color = color;
        self
    }

    #[allow(unused)]
    pub fn with_scale(mut self, numerator: u32, denominator: u32) -> Self {
        self.scale_numerator = numerator;
        self.scale_denominator = denominator;
        self
    }

    pub fn build(self) -> Text {
        Text {
            text: self.text,
            position: self.position,
            scale_numerator: self.scale_numerator,
            scale_denominator: self.scale_denominator,
            original_point: self.position.point,
            color: self.color,
        }
    }
}
