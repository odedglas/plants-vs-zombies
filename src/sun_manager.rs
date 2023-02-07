use crate::game::Game;
use crate::log;
use crate::model::{Position, SpriteType, TextOverlayData};
use crate::sprite::{Sprite, TextOverlay};

#[derive(Debug, Default)]
pub struct Features {
    pub update_score: bool,
    pub generate_sun: bool,
}

#[derive(Debug, Default)]
pub struct SunState {
    pub last_generated: f64,
    pub sun_interval: f64,
    pub score: usize,
    pub features: Features,
}

impl SunState {
    pub fn new() -> Self {
        SunState {
            score: 600,
            last_generated: 0.0,
            sun_interval: 15.0 * 1000.0,
            features: Features {
                update_score: false,
                generate_sun: false,
            },
        }
    }

    pub fn enable_sun(&mut self, enabled: bool) {
        self.features.generate_sun = enabled;
    }

    pub fn enable_score(&mut self, enabled: bool) {
        self.features.update_score = enabled;
    }
}

pub struct SunManager;

impl SunManager {
    pub fn tick(game: &mut Game) {
        let now = game.game_time.time;
        let mut state = &game.state.sun_state;

        if state.features.generate_sun {
            let should_generate = now - state.last_generated >= state.sun_interval;

            if should_generate {
                log!("Generating SUN!");
                game.state.sun_state.last_generated = now;
            }
        }
    }

    pub fn update_sun_score(game: &mut Game) {
        let mut state = &game.state.sun_state;
        let score = game.state.sun_state.score;

        if !state.features.update_score {
            return;
        }

        let sun_score = game.sprites.iter_mut().find(|sprite| sprite.name == "SunScore");

        if let Some(sun_score) = sun_score {
            sun_score.text_overlay = Some(TextOverlay::new(
                &TextOverlayData {
                    text: format!("{}", score),
                    size: 24,
                    offset: Some(Position::new(6.0, 14.0)),
                    location_type: Default::default(),
                    color: Some(String::from("Black")),
                },
                &sun_score,
            ));
        }
    }
}
