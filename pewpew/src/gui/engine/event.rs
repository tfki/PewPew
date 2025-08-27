use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Clone, Default)]
pub struct Event {
    fired: Arc<AtomicBool>,
}

impl Event {
    pub fn consume(&mut self) -> bool {
        if self.fired.load(Ordering::Relaxed) {
            self.reset();
            true
        } else {
            false
        }
    }

    pub fn trigger(&mut self) {
        self.fired.store(true, Ordering::Relaxed);
    }

    pub fn reset(&mut self) {
        self.fired.store(false, Ordering::Relaxed);
    }
}
