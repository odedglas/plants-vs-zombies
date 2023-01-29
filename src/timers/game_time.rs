use crate::timers::base_timer::Timer;

pub struct GameTime {
    pub time: f64,
    pub last_timestamp: f64,

    timer: Timer,
}

impl GameTime {
    pub fn new() -> GameTime {
        GameTime {
            time: 0.0,
            last_timestamp: 0.0,
            timer: Timer::new(1000.0),
        }
    }

    pub fn start(&mut self) {
        self.timer.start();
    }

    pub fn current_time(&mut self) -> f64 {
        // Tick between game frames.
        let now = self.timer.get_current_time();
        let elapsed = self.timer.get_elapsed_time(now);

        // Setting current time
        self.time = self.last_timestamp + elapsed;
        self.last_timestamp = self.time;

        // Reset timer for next tick
        self.timer.reset(None);

        self.time
    }
}
