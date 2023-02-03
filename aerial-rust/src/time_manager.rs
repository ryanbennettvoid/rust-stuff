use std::ops::{Div, Sub};
use std::thread::sleep;
use std::time::{Duration, Instant};

pub struct TimeManager {
    target_framerate: u32,
    frame_start_time: Instant,
    time_delta: Duration,
    frame: u64,
}

impl TimeManager {
    pub fn new(target_framerate: u32) -> Self {
        Self {
            target_framerate,
            frame_start_time: Instant::now(),
            time_delta: Duration::default(),
            frame: 0,
        }
    }

    pub fn on_frame_start(&mut self) {
        let now = Instant::now();
        self.time_delta = now.duration_since(self.frame_start_time);
        self.frame_start_time = now;
        self.frame += 1;
    }

    pub fn on_frame_end(&self) {
        let target_frame_duration = Duration::from_secs(1).div(self.target_framerate);
        let elapsed = Instant::now().duration_since(self.frame_start_time);
        if elapsed.lt(&target_frame_duration) {
            let remaining = target_frame_duration.sub(elapsed);
            sleep(remaining);
        }
    }

    pub fn get_delta(&self) -> Duration {
        self.time_delta
    }

    pub fn get_frame(&self) -> u64 {
        self.frame
    }
}
