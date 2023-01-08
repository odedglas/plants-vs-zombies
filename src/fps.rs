use crate::log;

#[derive(Debug, Default)]
pub struct Fps {
    current_time: f64,
    value: f64,
    display: u16,
    last_animation_frame_time: f64,
    last_fps_update_time: f64,
}

impl Fps {
    pub fn new() -> Fps {
        Fps {
            value: 60.0,
            display: 60,
            ..Default::default()
        }
    }

    pub fn format(value: f64) -> u16 {
        value as u16
    }

    pub fn calc(&mut self, now: f64) {
        self.current_time = now;
        self.value = (1.0 / (now - self.last_animation_frame_time)) * 1000.0;

        if now - self.last_fps_update_time > 1000.0 {
            self.last_fps_update_time = now;
            self.display = Fps::format(self.value);

            log!("Game fps: {}", &self.display)
        }
    }

    pub fn set(&mut self, now: f64) {
        self.last_animation_frame_time = now;
    }
}
