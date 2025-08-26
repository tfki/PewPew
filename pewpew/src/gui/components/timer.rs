use std::sync::Arc;
use hecs::{Entity, World};
use std::time::Duration;
use crate::gui::components::Action;

pub struct Timer {
    pub duration: Duration,
    pub looping: bool,
    pub action: Arc<Action>,
    pub next_activation_at_elapsed_game_time: Option<u128>,
}

pub struct Builder {
    pub duration: Duration,
    pub looping: bool,
    pub action: Arc<Action>,
}

impl Builder {
    pub fn new<A: Fn(Entity, &mut World) + Send + Sync + 'static>(
        duration: Duration,
        action: A,
    ) -> Self {
        Builder {
            duration,
            looping: false,
            action: Arc::new(action),
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
            action: self.action,
            next_activation_at_elapsed_game_time: None,
        }
    }
}
