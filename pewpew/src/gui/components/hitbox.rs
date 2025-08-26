use crate::gui::components::point_with_alignment::PointWithAlignment;

pub struct Hitbox {
    pub position: PointWithAlignment,
    pub width: u32,
    pub height: u32,
    pub z_index: i32,
}

pub struct Builder {
    position: PointWithAlignment,
    width: u32,
    height: u32,
    z_index: i32,
}

impl Builder {
    pub fn new(position: PointWithAlignment, width: u32, height: u32) -> Self {
        Builder {
            position,
            width,
            height,
            z_index: 0,
        }
    }

    pub fn with_z_index(mut self, z_index: i32) -> Self {
        self.z_index = z_index;
        self
    }

    pub fn build(self) -> Hitbox {
        Hitbox {
            position: self.position,
            width: self.width,
            height: self.height,
            z_index: self.z_index,
        }
    }
}
