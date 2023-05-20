use crate::features::GameFeatures;
use crate::game::Game;
use crate::location_builder::LocationBuilder;
use crate::model::{BehaviorType, Position, TextOverlayData};
use crate::resource_loader::ResourceKind;
use crate::sprite::{Animate, BehaviorManager, Sprite, TextOverlay, Walk};

#[derive(Debug, Default)]
pub struct SunState {
    pub last_generated: f64,
    pub sun_interval: f64,
    pub score: i32,
}

impl SunState {
    pub fn new() -> Self {
        SunState {
            score: 275,
            last_generated: 0.0,
            sun_interval: 15.0 * 1000.0,
        }
    }

    pub fn change_score(&mut self, score: i32) {
        self.score += score;
    }
}

pub struct SunManager;

impl SunManager {
    pub fn tick(game: &mut Game) {
        let now = game.game_time.time;
        let state = &game.state.sun_state;

        if GameFeatures::should_generate_sun() {
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
                    sun_score,
                ));
            }
        }
    }

    pub fn generate_sunflower_sun(game: &mut Game, source_position: Position) {
        let mut sun = Self::create_sun(
            game,
            Position::new(source_position.top - 50.0, source_position.left + 30.0),
        );

        sun.iter_mut().for_each(|sun_sprite| {
            let animate = BehaviorManager::get_sprite_behavior(sun_sprite, BehaviorType::Animate)
                .as_any()
                .downcast_mut::<Animate>()
                .unwrap();

            animate.set_max_cycles(12);

            let walk = BehaviorManager::get_sprite_behavior(sun_sprite, BehaviorType::Walk)
                .as_any()
                .downcast_mut::<Walk>()
                .unwrap();

            walk.velocity.y = -75.0;
            walk.velocity.x = 30.0;
        });

        // Triggers Interval which will reverse the direction at the end of it's first tick
        BehaviorManager::toggle_behaviors(&sun, &[BehaviorType::Interval], true, game.game_time.time);

        game.add_sprites(sun.as_mut());
    }

    pub fn reverse_sun(game: &mut Game, sprite_id: &String) {
        let now = game.game_time.time;
        let sun = game.get_sprite_by_id(sprite_id);

        // TODO - Refactor - First, we don't really need to toggle this behavior rather add "once" flag.
        // TODO - General concept can be swapped with "gravitation" behavior, endless interval which ticks velocity
        BehaviorManager::toggle_sprite_behaviors(sun, &[BehaviorType::Interval], false, now);

        let walk = BehaviorManager::get_sprite_behavior(sun, BehaviorType::Walk)
            .as_any()
            .downcast_mut::<Walk>()
            .unwrap();

        walk.velocity.y = 20.0;
        walk.velocity.x = 0.0;
    }

    pub fn collect_sun(game: &mut Game, sun_sprite_id: &String) {
        Self::change_score(game, 25);

        game.remove_sprite_by_id(sun_sprite_id);
    }

    pub fn change_score(game: &mut Game, addition: i32) {
        game.state.sun_state.change_score(addition);
    }

    fn generate_random_sun(game: &mut Game) {
        let mut sun = Self::create_sun(game, LocationBuilder::sun_location());

        game.add_sprites(sun.as_mut());
    }

    fn create_sun(game: &mut Game, position: Position) -> Vec<Sprite> {
        let mut sun_sprite =
            Sprite::create_sprites(vec!["Sun"], &ResourceKind::Interface, &game.resources);

        sun_sprite
            .iter_mut()
            .for_each(|sprite| sprite.update_position(position));

        BehaviorManager::toggle_behaviors(
            &sun_sprite,
            &[BehaviorType::Animate, BehaviorType::Walk],
            true,
            game.game_time.time,
        );

        sun_sprite
    }
}
