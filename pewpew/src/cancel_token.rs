use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct CancelToken {
    canceled: AtomicBool,
}

impl CancelToken {
    pub fn new() -> Arc<Self> {
        Arc::new(Self { canceled: AtomicBool::new(false)})
    }

    pub fn cancel(self: Arc<Self>) {
        self.canceled.store(true, Ordering::Relaxed);
    }

    pub fn was_canceled(&self) -> bool {
        self.canceled.load(Ordering::Relaxed)
    }
}
