#![no_std]

// use embassy for time primitives
type Instant = embassy_time::Instant;
type Duration = embassy_time::Duration;

/// provide relative time anchor
pub fn now() -> Instant {
    embassy_time::Instant::now()
}

/// Calculate duration between each step()
pub(crate) struct TimeBound {
    last_instant: Instant,
}
impl TimeBound {
    pub(crate) fn new() -> Self {
        Self {
            last_instant: now()
        }
    }

    /// upon each call, report the duration from the last call
    pub(crate) fn step(&mut self) -> Duration {
        let now = now();
        let ret = now - self.last_instant;
        self.last_instant = now;
        ret
    }
}

pub mod flight_controller;
