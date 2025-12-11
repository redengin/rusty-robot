/// use embassy primitives
use embassy_time::Instant;

/// Calculate duration between each step()
pub(crate) struct TimeBound {
    last_instant: Instant,
}
impl TimeBound {
    pub(crate) fn new() -> Self {
        Self {
            last_instant: Instant::now()
        }
    }

    /// upon each call, report the duration from the last call
    pub(crate) fn step(&mut self) -> core::time::Duration {
        let now = Instant::now();
        let ret = now - self.last_instant;
        self.last_instant = now;
        ret.into()
    }
}

