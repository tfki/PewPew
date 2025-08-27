use std::time::Duration;
use crate::gui::engine::event::Event;

pub struct Timer {
    pub duration: Duration,
    pub looping: bool,
    pub event: Event,
    pub next_activation_at_elapsed_game_time: Option<u128>,
}

pub struct Builder {
    pub duration: Duration,
    pub looping: bool,
    pub event: Event,
}

impl Builder {
    pub fn new(
        duration: Duration,
        event: Event,
    ) -> Self {
        Builder {
            duration,
            looping: false,
            event,
        }
    }

    pub fn looping(mut self) -> Self {
        self.looping = true;
        self
    }

    pub fn build(self) -> Timer {
        Timer {
            duration: self.duration,
            looping: self.looping,
            event: self.event,
            next_activation_at_elapsed_game_time: None,
        }
    }
}
