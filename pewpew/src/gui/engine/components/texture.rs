use std::time::Duration;
use crate::gui::engine::components::Point;
use crate::gui::engine::components::point_with_alignment::PointWithAlignment;
use crate::gui::engine::event::Event;

#[derive(Clone)]
pub enum AnimationEndBehavior {
    Freeze,
    Loop,
}

#[derive(Clone)]
pub struct Texture {
    pub position: PointWithAlignment,

    /// as a texture is movable by a movement component
    /// its original point needs to be stored
    /// as the movement function outputs a point difference to an original position
    pub original_point: Point,
    pub animation_end_behavior: AnimationEndBehavior,
    pub z_index: i32,
    pub image_id: usize,
    pub num_frames: u32,
    pub current_keyframe: u32,
    pub scale: f32,
    pub flip_horizontally: bool,
    pub flip_vertically: bool,
    pub rotation_deg: f64,
    pub keyframe_duration: Duration,
    pub next_keyframe_switch_at_elapsed_game_time: Option<u128>,

    /// triggered every time, animated texture reaches its end
    pub animation_end_event: Option<Event>,

    /// triggered if part of the texture is outside the viewport
    pub at_viewport_edge_event: Option<Event>,

    /// triggered if texture is entirely outside the viewport
    pub outside_viewport_event: Option<Event>,
}

pub struct Builder {
    position: PointWithAlignment,
    animation_end_behavior: AnimationEndBehavior,
    z_index: i32,
    image_id: usize,
    num_frames: u32,
    current_frame: u32,
    scale: f32,
    flip_horizontally: bool,
    flip_vertically: bool,
    rotation_deg: f64,
    frame_advance_interval: Option<Duration>,
    animation_end_event: Option<Event>,
    at_viewport_edge_event: Option<Event>,
    outside_viewport_event: Option<Event>,
}

impl Builder {
    pub fn new(image_id: usize, position: PointWithAlignment) -> Self {
        Builder {
            position,
            image_id,
            animation_end_behavior: AnimationEndBehavior::Loop,
            z_index: 0,
            num_frames: 1,
            current_frame: 0,
            flip_vertically: false,
            flip_horizontally: false,
            scale: 1.0,
            rotation_deg: 0.0,
            frame_advance_interval: None,
            animation_end_event: None,
            at_viewport_edge_event: None,
            outside_viewport_event: None,
        }
    }

    pub fn with_animation_end_behavior(mut self, animation_end_behavior: AnimationEndBehavior) -> Self {
        self.animation_end_behavior = animation_end_behavior;
        self
    }

    pub fn on_animation_end(mut self, event: Event) -> Self {
        self.animation_end_event = Some(event);
        self
    }

    #[allow(unused)]
    pub fn on_at_viewport_edge(mut self, event: Event) -> Self {
        self.at_viewport_edge_event = Some(event);
        self
    }

    #[allow(unused)]
    pub fn on_outside_viewport(mut self, event: Event) -> Self {
        self.outside_viewport_event = Some(event);
        self
    }

    pub fn with_num_frames(mut self, num_frames: u32) -> Self {
        self.num_frames = num_frames;
        self
    }

    pub fn with_z_index(mut self, z_index: i32) -> Self {
        self.z_index = z_index;
        self
    }

    pub fn with_scale(mut self, scale: f32) -> Self {
        self.scale = scale;
        self
    }

    #[allow(unused)]
    pub fn with_rotation_deg(mut self, rotation_deg: f64) -> Self {
        self.rotation_deg = rotation_deg;
        self
    }

    #[allow(unused)]
    pub fn with_horizontal_flip(mut self) -> Self {
        self.flip_horizontally = true;
        self
    }

    pub fn with_vertical_flip(mut self) -> Self {
        self.flip_vertically = true;
        self
    }

    pub fn with_frame_advance_interval(mut self, frame_advance_interval: Duration) -> Self {
        self.frame_advance_interval = Some(frame_advance_interval);
        self
    }

    pub fn build(self) -> Texture {
        Texture {
            position: self.position,
            animation_end_behavior: self.animation_end_behavior,
            image_id: self.image_id,
            z_index: self.z_index,
            num_frames: self.num_frames,
            current_keyframe: self.current_frame,
            scale: self.scale,
            flip_horizontally: self.flip_vertically,
            flip_vertically: self.flip_horizontally,
            rotation_deg: self.rotation_deg,
            keyframe_duration: self
                .frame_advance_interval
                .unwrap_or(Duration::from_secs(u64::MAX)),
            next_keyframe_switch_at_elapsed_game_time: None,
            animation_end_event: self.animation_end_event,
            at_viewport_edge_event: self.at_viewport_edge_event,
            outside_viewport_event: self.outside_viewport_event,
            original_point: self.position.point,
        }
    }
}
