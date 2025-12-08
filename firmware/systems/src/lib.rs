#![no_std]

// use embassy for time primitives
type Instant = embassy_time::Instant;
type Duration = embassy_time::Duration;

/// provide relative time anchor
pub fn now() -> Instant {
    embassy_time::Instant::now()
}

pub struct TimeBound {
    last_instant: Instant,
}
impl TimeBound {
    pub fn new() -> Self {
        Self {
            last_instant: now()
        }
    }

    /// upon each call, report the duration from the last call
    pub fn step(&mut self) -> Duration {
        let now = now();
        let ret = now - self.last_instant;
        self.last_instant = now;
        ret
    }
}

pub mod flight_controller;
