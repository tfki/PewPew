use sdl2::rect::Rect;
use crate::gui::engine::components::Point;

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum VAlign {
    Top,
    Center,
    Bottom,
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum HAlign {
    Left,
    Center,
    Right,
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct PointWithAlignment {
    pub point: Point,
    pub v_align: VAlign,
    pub h_align: HAlign,
}

impl PointWithAlignment {
    pub fn align_rect(&self, width: u32, height: u32) -> Rect {
        let x = match self.h_align {
            HAlign::Left => self.point.x,
            HAlign::Center => self.point.x - width as i32 / 2,
            HAlign::Right => self.point.x - width as i32,
        };

        let y = match self.v_align {
            VAlign::Top => self.point.y,
            VAlign::Center => self.point.y - height as i32 / 2,
            VAlign::Bottom => self.point.y - height as i32,
        };

        Rect::new(x, y, width, height)
    }

    pub fn new_top_left(point: Point) -> Self {
        PointWithAlignment {
            point,
            v_align: VAlign::Top,
            h_align: HAlign::Left,
        }
    }

    pub fn new_center(point: Point) -> Self {
        PointWithAlignment {
            point,
            v_align: VAlign::Center,
            h_align: HAlign::Center,
        }
    }
}
