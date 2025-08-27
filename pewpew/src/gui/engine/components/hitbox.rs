use crate::gui::engine::components::Point;
use crate::gui::engine::components::point_with_alignment::PointWithAlignment;
use crate::gui::engine::event::Event;

pub struct Hitbox {
    pub position: PointWithAlignment,

    /// as a hitbox is movable by a movement component
    /// its original point needs to be stored
    /// as the movement function outputs a point difference to an original position
    pub original_point: Point,
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

    #[allow(unused)]
    pub fn on_hit(mut self, event: Event) -> Self {
        self.hit_event = Some(event);
        self
    }

    #[allow(unused)]
    pub fn with_z_index(mut self, z_index: i32) -> Self {
        self.z_index = z_index;
        self
    }

    pub fn build(self) -> Hitbox {
        Hitbox {
            position: self.position,
            original_point: self.position.point,
            width: self.width,
            height: self.height,
            z_index: self.z_index,
            hit_event: self.hit_event,
        }
    }
}
