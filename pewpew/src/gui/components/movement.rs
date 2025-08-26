use crate::gui::components::Point;
use std::time::Duration;

pub type By = Point;
pub struct Movement {
    pub by: By,
    pub every: Duration,
    pub next_movement_at_elapsed_game_time: Option<u128>,
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
            next_movement_at_elapsed_game_time: None,
        }
    }
}
