use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Clone)]
pub struct CancelToken {
    canceled: Arc<AtomicBool>,
}

impl CancelToken {
    pub fn cancel(&self) {
        self.canceled.store(true, Ordering::Relaxed);
    }

    pub fn was_canceled(&self) -> bool {
        self.canceled.load(Ordering::Relaxed)
    }
}

impl Default for CancelToken {
    fn default() -> Self {
        CancelToken { canceled: Arc::new(AtomicBool::new(false)) }
    }
}

impl Drop for CancelToken {
    fn drop(&mut self) {
        self.cancel();
    }
}
