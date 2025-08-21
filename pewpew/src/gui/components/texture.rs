use std::time::{Duration, SystemTime};
use crate::gui::components::Point;

pub struct Texture {
    pub anchor: Point,
    pub image_id: usize,
    pub num_frames: u32,
    pub current_keyframe: u32,
    pub repeat: bool,
    pub scale: f32,
    pub flip_horizontally: bool,
    pub flip_vertically: bool,
    pub rotation_deg: f64,
    pub keyframe_duration: Duration,
    pub last_keyframe_change_time: Option<SystemTime>,
}

pub struct Builder {
    anchor: Point,
    image_id: usize,
    num_frames: u32,
    current_frame: u32,
    looping: bool,
    scale: f32,
    flip_horizontally: bool,
    flip_vertically: bool,
    rotation_deg: f64,
    frame_advance_interval: Option<Duration>,
}

impl Builder {
    pub fn new(image_id: usize, anchor: Point) -> Self {
        Builder {
            anchor,
            image_id,
            num_frames: 1,
            current_frame: 0,
            looping: false,
            flip_vertically: false,
            flip_horizontally: false,
            scale: 1.0,
            rotation_deg: 0.0,
            frame_advance_interval: None,
        }
    }

    pub fn with_num_frames(mut self, num_frames: u32) -> Self {
        self.num_frames = num_frames;
        self
    }

    pub fn looping(mut self) -> Self {
        self.looping = true;
        self
    }

    pub fn with_scale(mut self, scale: f32) -> Self {
        self.scale = scale;
        self
    }

    pub fn with_rotation_deg(mut self, rotation_deg: f64) -> Self {
        self.rotation_deg = rotation_deg;
        self
    }

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
            anchor: self.anchor,
            image_id: self.image_id,
            num_frames: self.num_frames,
            current_keyframe: self.current_frame,
            repeat: self.looping,
            scale: self.scale,
            flip_horizontally: self.flip_vertically,
            flip_vertically: self.flip_horizontally,
            rotation_deg: self.rotation_deg,
            keyframe_duration: self.frame_advance_interval.unwrap_or(Duration::from_secs(u64::MAX)),
            last_keyframe_change_time: None,
        }
    }
}
