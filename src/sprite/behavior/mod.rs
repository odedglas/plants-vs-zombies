pub use animate::Animate;
pub use base::Behavior;
pub use click::Click;
pub use collision::{Collision, CollisionState};
pub use hover::Hover;
pub use interval::Interval;
pub use scroll::Scroll;
pub use walk::Walk;
use web_sys::CanvasRenderingContext2d;

use crate::model::{BehaviorData, BehaviorType, GameInteraction, Position};
use crate::sprite::behavior::drag::Drag;
use crate::sprite::{Sprite, SpriteMutation};
use crate::timers::GameTime;

mod animate;
mod base;
mod click;
mod collision;
mod drag;
mod hover;
mod interval;
mod scroll;
mod walk;

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
            BehaviorType::Walk => Box::new(Walk::new(data.distance, data.velocity.unwrap())),
            BehaviorType::Drag => Box::new(Drag::new(data.callback.unwrap())),
            BehaviorType::Interval => Box::new(Interval::new(data.interval.unwrap(), data.callback)),
            BehaviorType::Collision => Box::new(Collision::new(
                data.collision_margin.unwrap_or_default(),
            )),
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
            .filter_map(|behavior| {
                behavior.execute(sprite, time.time, time.last_timestamp, position, context)
            })
            .collect()
    }

    pub fn toggle_behaviors(
        sprites: &Vec<Sprite>,
        behavior_types: &[BehaviorType],
        should_run: bool,
        now: f64,
    ) {
        sprites.iter().for_each(|sprite| {
            Self::toggle_sprite_behaviors(sprite, behavior_types, should_run, now)
        });
    }

    pub fn toggle_sprite_behaviors(
        sprite: &Sprite,
        behavior_types: &[BehaviorType],
        should_run: bool,
        now: f64,
    ) {
        sprite
            .mutable_behaviors()
            .iter_mut()
            .filter(|behavior| behavior.is_running() != should_run)
            .filter(|behavior| behavior_types.contains(&behavior.name()))
            .for_each(|behavior| behavior.toggle(should_run, now));
    }

    pub fn get_sprite_behavior(
        sprite: &mut Sprite,
        behavior: BehaviorType,
    ) -> &mut Box<dyn Behavior> {
        let sprite_id = sprite.id.clone();
        Self::find_sprite_behavior(sprite, behavior).unwrap_or_else(|| panic!("[BehaviorManager] Cannot GET Sprite behavior: {:?} / {}",
            behavior, sprite_id))
    }

    pub fn find_sprite_behavior(
        sprite: &mut Sprite,
        behavior: BehaviorType,
    ) -> Option<&mut Box<dyn Behavior>> {
        sprite
            .behaviors
            .get_mut()
            .iter_mut()
            .find(|sprite_behavior| behavior == sprite_behavior.name())
    }

    pub fn collect_interactions(sprite: &Sprite) -> Vec<GameInteraction> {
        let interactions = sprite
            .mutable_behaviors()
            .iter_mut()
            .filter_map(|behavior| behavior.get_interaction())
            .collect();

        sprite.mutable_behaviors().iter_mut().for_each(|behavior| {
            behavior.clean_interaction();
        });

        interactions
    }
}
