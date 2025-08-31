use std::sync::Arc;
use hecs::World;
use crate::gui::engine::event::Event;

pub struct Condition {
    pub f: Arc<dyn 'static + Send + Sync + Fn(&World) -> bool>,
    pub event: Event
}

impl Condition {
    pub fn new<F: 'static + Send + Sync + Fn(&World) -> bool>(f: F, event: Event) -> Self {
        Self { f: Arc::new(f), event }
    }
}
