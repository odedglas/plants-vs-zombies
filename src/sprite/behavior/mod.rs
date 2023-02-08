mod animate;
mod base;
mod click;
mod hover;
mod scroll;
mod walk;

pub use animate::Animate;
pub use base::Behavior;
pub use click::Click;
pub use hover::Hover;
pub use scroll::Scroll;
pub use walk::Walk;
use web_sys::CanvasRenderingContext2d;

use crate::model::{BehaviorData, BehaviorType, GameInteraction, Position};
use crate::sprite::{Sprite, SpriteMutation};
use crate::timers::GameTime;

pub struct BehaviorManager;

impl BehaviorManager {
    pub fn create(data: &BehaviorData, sprite_id: String) -> Box<dyn Behavior> {
        let behavior_type = BehaviorType::from_string(&data.name);

        let mut behavior: Box<dyn Behavior> = match behavior_type {
            BehaviorType::Click => Box::new(Click::new(data.callback.unwrap())),
            BehaviorType::Animate => Box::new(Animate::new(
                data.rate,
                data.callback,
                data.callback_delay,
                data.max_cycles,
            )),
            BehaviorType::Hover => Box::new(Hover::new()),
            BehaviorType::Scroll => Box::new(Scroll::new(
                data.distance,
                data.rate,
                data.callback.unwrap(),
            )),
            BehaviorType::Walk => Box::new(Walk::new(data.distance, data.velocity.unwrap().clone())),
        };

        behavior.set_sprite_id(sprite_id);

        behavior
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
        sprites: &Vec<Sprite>,
        behavior_types: &[BehaviorType],
        should_run: bool,
        now: f64,
    ) {
        sprites.iter().for_each(|sprite| {
            sprite
                .mutable_behaviors()
                .iter_mut()
                .filter(|behavior| behavior_types.contains(&behavior.name()))
                .for_each(|behavior| behavior.toggle(should_run, now));
        });
    }

    pub fn get_sprite_behavior(
        sprite: &mut Sprite,
        behavior: BehaviorType,
    ) -> &mut Box<dyn Behavior> {
        let behavior = sprite
            .behaviors
            .get_mut()
            .iter_mut()
            .find(|sprite_behavior| behavior == sprite_behavior.name())
            .expect(&format!(
                "[BehaviorManager] Cannot find Sprite behavior: {:?}",
                behavior
            ));

        behavior
    }

    pub fn collect_interactions(sprite: &Sprite) -> Vec<GameInteraction> {
        let interactions = sprite
            .mutable_behaviors()
            .iter_mut()
            .map(|behavior| behavior.get_interaction())
            .filter_map(|interaction| interaction)
            .collect();

        sprite.mutable_behaviors().iter_mut().for_each(|behavior| {
            behavior.clean_interaction();
        });

        interactions
    }
}
