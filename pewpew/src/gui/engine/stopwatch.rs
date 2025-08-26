use std::time::{Duration, Instant};
use log::debug;

pub struct Stopwatch {
    last_start: Option<Instant>,
    before_start: Duration,
}

impl Stopwatch {
    pub fn new_paused() -> Self {
        Stopwatch {
            last_start: None,
            before_start: Duration::default(),
        }
    }

    pub fn pause(&mut self) {
        if let Some(start) = self.last_start.take() {
            debug!(target: "Gui Thread", "time was resumed at {}ms elapsed", self.elapsed_ms());
            let now = Instant::now();
            let diff = now.duration_since(start);

            self.before_start += diff;
            self.last_start = None;
        }
    }

    pub fn resume(&mut self) {
        if self.last_start.is_none() {
            debug!(target: "Gui Thread", "time was stopped at {}ms elapsed", self.elapsed_ms());
            self.last_start = Some(Instant::now());
        }
    }

    pub fn elapsed_ms(&self) -> u128 {
        (self.before_start
            + match self.last_start {
                None => Duration::default(),
                Some(last_start) => Instant::now().duration_since(last_start),
            }).as_millis()
    }
}
