use crate::gui::components::Point;
use std::time::{Duration, SystemTime};

pub type By = Point;
pub struct Movement {
    pub by: By,
    pub every: Duration,
    pub last_movement_time: Option<SystemTime>,
}

pub struct Builder {
    by: By,
    every: Duration,
}

impl Builder {
    pub fn new(by: By, every: Duration) -> Self {
        Builder { by, every }
    }

    pub fn build(self) -> Movement {
        Movement {
            by: self.by,
            every: self.every,
            last_movement_time: None,
        }
    }
}
