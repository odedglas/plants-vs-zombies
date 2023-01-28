mod base;
mod click;
mod hover;

use std::slice::Iter;

pub use base::Behavior;
pub use click::Click;
pub use hover::Hover;
use web_sys::CanvasRenderingContext2d;

use crate::model::{BehaviorData, BehaviorType, GameInteraction, Position};
use crate::sprite::{Sprite, SpriteMutation};
use crate::timers::GameTime;

pub struct BehaviorManager;

impl BehaviorManager {
    pub fn create(data: &BehaviorData) -> Box<dyn Behavior> {
        let behavior_type = BehaviorType::from_string(&data.name);

        match behavior_type {
            BehaviorType::Click => Box::new(Click::new()),
            BehaviorType::Hover => Box::new(Hover::new()),
        }
    }

    pub fn run(
        sprite: &Sprite,
        time: &GameTime,
        position: &Position,
        context: &CanvasRenderingContext2d,
    ) -> Vec<SpriteMutation> {
        sprite
            .mutable_behaviors()
            .iter_mut()
            .filter(|behavior| behavior.is_running())
            .map(|behavior| {
                behavior.execute(sprite, time.time, time.last_timestamp, position, context)
            })
            .filter_map(|mutation| mutation)
            .collect()
    }

    pub fn toggle_behaviors(
        sprites: Iter<Sprite>,
        behavior_types: &[BehaviorType],
        should_run: bool,
        now: f64,
    ) {
        sprites.for_each(|sprite| {
            sprite
                .mutable_behaviors()
                .iter_mut()
                .filter(|behavior| behavior_types.contains(&behavior.name()))
                .for_each(|behavior| behavior.toggle(should_run, now));
        });
    }

    pub fn collect_interactions(sprites: &Sprite) -> Vec<GameInteraction> {
        let interactions = sprites
            .mutable_behaviors()
            .iter_mut()
            .map(|behavior| behavior.get_interaction())
            .filter_map(|interaction| interaction)
            .collect();

        sprites.mutable_behaviors().iter_mut().for_each(|behavior| {
            behavior.clean_interaction();
        });

        interactions
    }
}
