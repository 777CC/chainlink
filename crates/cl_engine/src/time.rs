//! Frame timing utilities.

use std::time::Instant;

/// Tracks elapsed time and computes per-frame deltas.
pub struct GameTime {
    start:      Instant,
    last_frame: Instant,
    /// Elapsed seconds since the previous frame.
    pub delta:  f64,
    /// Total elapsed seconds since engine start.
    pub elapsed: f64,
    /// Frame count since start.
    pub frame:  u64,
}

impl GameTime {
    pub fn new() -> Self {
        let now = Instant::now();
        Self {
            start:      now,
            last_frame: now,
            delta:      0.0,
            elapsed:    0.0,
            frame:      0,
        }
    }

    /// Call once per frame to advance the clock.
    pub fn tick(&mut self) {
        let now        = Instant::now();
        self.delta     = now.duration_since(self.last_frame).as_secs_f64();
        self.elapsed   = now.duration_since(self.start).as_secs_f64();
        self.last_frame = now;
        self.frame     += 1;
    }

    /// Frames per second (instantaneous).
    pub fn fps(&self) -> f64 {
        if self.delta > 0.0 { 1.0 / self.delta } else { 0.0 }
    }
}

impl Default for GameTime {
    fn default() -> Self { Self::new() }
}
