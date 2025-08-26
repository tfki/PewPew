use crate::gui::engine::components::point_with_alignment::PointWithAlignment;
use crate::gui::engine::event::Event;

pub struct Hitbox {
    pub position: PointWithAlignment,
    pub width: u32,
    pub height: u32,
    pub z_index: i32,
    pub hit_event: Option<Event>,
}

pub struct Builder {
    position: PointWithAlignment,
    width: u32,
    height: u32,
    z_index: i32,
    hit_event: Option<Event>,
}

impl Builder {
    pub fn new(position: PointWithAlignment, width: u32, height: u32) -> Self {
        Builder {
            position,
            width,
            height,
            z_index: 0,
            hit_event: None,
        }
    }

    pub fn on_hit(mut self, event: Event) -> Self {
        self.hit_event = Some(event);
        self
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
            hit_event: self.hit_event,
        }
    }
}
