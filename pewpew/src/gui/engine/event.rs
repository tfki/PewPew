use std::sync::{Arc, Mutex};
use std::time::Duration;
use crate::gui::engine::components::timer;
use crate::gui::engine::components::timer::Timer;

pub struct Event {
    fired: Arc<Mutex<Vec<bool>>>,
    my_idx: usize,
}

impl Default for Event {
    fn default() -> Self {
        Self {
            fired: Arc::new(Mutex::new(vec![false])),
            my_idx: 0,
        }
    }
}

impl Event {
    #[allow(unused)]
    pub fn trigger_after(self, duration: Duration) -> Timer {
        timer::Builder::new(duration, self).build()
    }

    pub fn trigger_every(self, duration: Duration) -> Timer {
        timer::Builder::new(duration, self).looping().build()
    }

    pub fn consume(&mut self) -> bool {
        // we are single threaded, so unwrap is ok
        let mut locked = self.fired.lock().unwrap();
        if locked[self.my_idx] {
            locked[self.my_idx] = false;
            true
        } else {
            false
        }
    }

    pub fn trigger(&mut self) {
        // we are single threaded, so unwrap is ok
        for b in self.fired.lock().unwrap().iter_mut() {
            *b = true;
        }
    }

    #[allow(unused)]
    pub fn reset(&mut self) {
        // we are single threaded, so unwrap is ok
        self.fired.lock().unwrap()[self.my_idx] = false;
    }
}

impl Clone for Event {
    fn clone(&self) -> Self {
        let clone_idx = {
            let mut locked = self.fired.lock().unwrap();
            locked.push(false);
            locked.len() - 1
        };
        Event {
            fired: self.fired.clone(),
            my_idx: clone_idx,
        }
    }
}
