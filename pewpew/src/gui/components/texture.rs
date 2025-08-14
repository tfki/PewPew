use std::time::{Duration, SystemTime};

pub struct Texture {
    pub image_id: usize,
    pub num_frames: u32,
    pub current_frame: u32,
    pub repeat: bool,
    pub scale: f32,
    pub rotation_deg: f64,
    pub frame_advance_interval: Duration,
    pub last_frame_time: Option<SystemTime>,
}

pub struct Builder {
    image_id: usize,
    num_frames: u32,
    current_frame: u32,
    repeat: bool,
    scale: f32,
    rotation_deg: f64,
    frame_advance_interval: Option<Duration>,
    last_frame_time: Option<SystemTime>,
}

impl Builder {
    pub fn new(image_id: usize) -> Self {
        Builder {
            image_id,
            num_frames: 1,
            current_frame: 0,
            repeat: false,
            scale: 1.0,
            rotation_deg: 0.0,
            frame_advance_interval: None,
            last_frame_time: None,
        }
    }

    pub fn with_num_frames(mut self, num_frames: u32) -> Self {
        self.num_frames = num_frames;
        self
    }

    pub fn looping(mut self) -> Self {
        self.repeat = true;
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

    pub fn with_frame_advance_interval(mut self, frame_advance_interval: Duration) -> Self {
        self.frame_advance_interval = Some(frame_advance_interval);
        self
    }

    pub fn build(self) -> Texture {
        Texture {
            image_id: self.image_id,
            num_frames: self.num_frames,
            current_frame: self.current_frame,
            repeat: self.repeat,
            scale: self.scale,
            rotation_deg: self.rotation_deg,
            frame_advance_interval: self.frame_advance_interval.unwrap_or(Duration::from_secs(u64::MAX)),
            last_frame_time: None,
        }
    }
}
