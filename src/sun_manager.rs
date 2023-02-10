use crate::features::GameFeatures;
use crate::game::Game;
use crate::location_builder::LocationBuilder;
use crate::model::{BehaviorType, Position, TextOverlayData};
use crate::resource_loader::ResourceKind;
use crate::sprite::{BehaviorManager, Sprite, TextOverlay};

#[derive(Debug, Default)]
pub struct SunState {
    pub last_generated: f64,
    pub sun_interval: f64,
    pub score: i32,
}

impl SunState {
    pub fn new() -> Self {
        SunState {
            score: 600,
            last_generated: 0.0,
            sun_interval: 15.0 * 1000.0,
        }
    }

    pub fn add_score(&mut self, score: i32) {
        self.score += score;
    }
}

pub struct SunManager;

impl SunManager {
    pub fn tick(game: &mut Game) {
        let now = game.game_time.time;
        let state = &game.state.sun_state;
        let mut last_generated = state.last_generated;

        if GameFeatures::should_generate_sun() {
            // Avoid first tick
            if last_generated == 0.0 {
                last_generated = now
            }

            let should_generate = now - state.last_generated >= state.sun_interval;

            if should_generate {
                game.state.sun_state.last_generated = now;

                Self::generate_random_sun(game);
            }
        } else {
            // Resets last generated point
            game.state.sun_state.last_generated = 0.0
        }
    }

    pub fn update_sun_score(game: &mut Game) {
        let score = game.state.sun_state.score;

        if GameFeatures::should_update_sun_score() {
            let sun_score = game
                .sprites
                .iter_mut()
                .find(|sprite| sprite.name == "SunScore");

            if let Some(sun_score) = sun_score {
                sun_score.text_overlay = Some(TextOverlay::new(
                    &TextOverlayData {
                        text: format!("{}", score),
                        size: 24,
                        offset: Some(Position::new(6.0, 14.0)),
                        location_type: Default::default(),
                        color: Some(String::from("black")),
                    },
                    &sun_score,
                ));
            }
        }
    }

    fn generate_random_sun(game: &mut Game) {
        let mut sun_sprite =
            Sprite::create_sprites(vec!["Sun"], &ResourceKind::Interface, &game.resources);

        sun_sprite
            .iter_mut()
            .for_each(|sprite| sprite.update_position(LocationBuilder::sun_location()));

        BehaviorManager::toggle_behaviors(
            &sun_sprite,
            &[BehaviorType::Animate, BehaviorType::Walk],
            true,
            game.game_time.time,
        );

        game.add_sprites(sun_sprite.as_mut());
    }
}
