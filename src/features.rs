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

    pub fn enable_board_lines() {
        FEATURES.lock().unwrap().draw_board_lines = true
    }
}

pub static FEATURES: Mutex<GameFeatures> = Mutex::new(GameFeatures {
    draw_board_lines: false,
    update_sun_score: false,
    generate_sun: false,
});
