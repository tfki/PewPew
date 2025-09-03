use crate::gui::engine::components::timer;
use crate::gui::engine::components::timer::Timer;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

// events can be cloned infinitely often
// if these clones are all quickly dropped again
// the runtime and memory consumption of the event should stay the same
// thus the shared data contains a map of active ids mapped to indices
// in the vector
// this allows for cleanup when an event is dropped

#[derive(Default)]
struct SharedEventData {
    trigger_counts: Vec<u32>,
    id_idx_map: HashMap<usize, usize>,
    id_counter: usize,
}

impl SharedEventData {
    pub fn trigger(&mut self) {
        for x in &mut self.trigger_counts {
            *x += 1;
        }
    }

    pub fn consume_all(&mut self, idx: usize) -> u32 {
        let trigger_count = self.trigger_counts[self.id_idx_map[&idx]];
        self.trigger_counts[self.id_idx_map[&idx]] = 0;

        trigger_count
    }

    pub fn add_subscriber(&mut self) -> usize {
        let new_id = self.id_counter;
        self.id_counter += 1;

        let new_idx = self.trigger_counts.len();
        self.trigger_counts.push(0);

        if self.id_idx_map.insert(new_id, new_idx).is_some() {
            unreachable!();
        }

        new_id
    }

    pub fn remove_subscriber(&mut self, id: usize) {
        let rm_idx = self.id_idx_map.remove(&id).unwrap();

        self.trigger_counts.remove(rm_idx);

        for (_, idx) in self.id_idx_map.iter_mut() {
            if *idx > rm_idx {
                *idx -= 1;
            }
        }
    }
}

pub struct Event {
    shared_data: Arc<Mutex<SharedEventData>>,
    my_id: usize,
}

impl Default for Event {
    fn default() -> Self {
        let mut shared_data = SharedEventData::default();
        let my_id = shared_data.add_subscriber();
        Self {
            shared_data: Arc::new(Mutex::new(shared_data)),
            my_id,
        }
    }
}

impl Drop for Event {
    fn drop(&mut self) {
        let mut locked = self.shared_data.lock().unwrap();
        locked.remove_subscriber(self.my_id);
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

    pub fn consume_all(&mut self) -> u32 {
        // we are single threaded, so unwrap is ok
        let mut locked = self.shared_data.lock().unwrap();
        locked.consume_all(self.my_id)
    }

    pub fn trigger(&mut self) {
        // we are single threaded, so unwrap is ok
        self.shared_data.lock().unwrap().trigger();
    }
}

impl Clone for Event {
    fn clone(&self) -> Self {
        let my_id = { self.shared_data.lock().unwrap().add_subscriber() };
        Event {
            shared_data: self.shared_data.clone(),
            my_id,
        }
    }
}
