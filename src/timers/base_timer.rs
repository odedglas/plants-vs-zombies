use crate::web_utils::window_time;

#[derive(Debug, Default)]
pub struct Timer {
    elapsed: f64,
    pub running: bool,
    start_time: f64,
}

impl Timer {
    pub fn new(elapsed: f64) -> Self {
        Timer {
            elapsed,
            running: false,
            start_time: 0.0,
        }
    }

    pub fn set_elapsed(&mut self, elapsed: f64) {
        self.elapsed = elapsed;
    }

    pub fn get_current_time(&self) -> f64 {
        window_time()
    }

    pub fn start(&mut self) {
        self.running = true;
        self.start_time = self.get_current_time();
    }

    pub fn reset(&mut self, now: Option<f64>) {
        self.start_time = match now {
            Some(now) => now,
            None => self.get_current_time(),
        };
    }

    pub fn get_elapsed_time(&self, now: f64) -> f64 {
        match self.running {
            true => now - self.start_time,
            false => self.elapsed,
        }
    }

    pub fn expired(&self, now: f64) -> bool {
        if !self.running {
            return false;
        }

        self.get_elapsed_time(now) >= self.elapsed
    }
}
