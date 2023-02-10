use std::sync::Mutex;

#[derive(Debug, Default)]
pub struct GameFeatures {
    pub draw_board_lines: bool,
    pub update_sun_score: bool,
    pub generate_sun: bool,
}

impl GameFeatures {
    pub fn show_board_lines() -> bool {
        FEATURES.lock().unwrap().draw_board_lines
    }

    pub fn enable_board_lines(enabled: bool) {
        FEATURES.lock().unwrap().draw_board_lines = enabled
    }

    pub fn should_update_sun_score() -> bool {
        FEATURES.lock().unwrap().update_sun_score
    }

    pub fn enable_update_sun_score(enabled: bool) {
        FEATURES.lock().unwrap().update_sun_score = enabled
    }

    pub fn should_generate_sun() -> bool {
        FEATURES.lock().unwrap().generate_sun
    }

    pub fn enable_generate_sun(enabled: bool) {
        FEATURES.lock().unwrap().generate_sun = enabled
    }
}

pub static FEATURES: Mutex<GameFeatures> = Mutex::new(GameFeatures {
    draw_board_lines: false,
    update_sun_score: false,
    generate_sun: false,
});
