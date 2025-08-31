use crate::gui::engine::components::Point;
use std::sync::Arc;


#[derive(Clone)]
pub struct Movement {
    pub f: Arc<dyn Send + Sync + Fn(u128) -> Point>,
    pub first_invocation_game_time: Option<u128>,
}

impl Movement {
    pub fn new<F: 'static + Send + Sync + Fn(u128) -> Point>(f: F) -> Movement {
        Movement {
            f: Arc::new(f),
            first_invocation_game_time: None,
        }
    }

    pub fn id() -> Movement {
        Movement::new(|i| Point{ x: 0, y: 0 })
    }
}
