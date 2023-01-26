mod base;
mod click;
mod hover;

use std::slice::IterMut;

pub use base::Behavior;
pub use click::Click;
pub use hover::Hover;
use web_sys::CanvasRenderingContext2d;

use crate::log;
use crate::model::{BehaviorType, Position};
use crate::sprite::{Sprite, SpriteMutation};
use crate::timers::GameTime;

pub struct BehaviorManager;

impl BehaviorManager {
    pub fn create(id: &str, name: &str) -> Box<dyn Behavior> {
        let behavior_id = String::from(id);

        match name {
            "Click" => Box::new(Click::new(behavior_id)),
            _ => Box::new(Hover::new(behavior_id)),
        }
    }

    pub fn run(
        sprite: &mut Sprite,
        time: &GameTime,
        position: &Position,
        context: &CanvasRenderingContext2d,
    ) -> Vec<SpriteMutation> {
        sprite
            .behaviors
            .iter_mut()
            .filter(|behavior| behavior.is_running())
            .map(|behavior| {
                log!("Working {}", sprite.id);
                behavior.execute(time.time, time.last_timestamp, position, context)
            })
            .filter_map(|mutation| mutation)
            .collect()
    }

    pub fn toggle_behaviors(
        sprite: IterMut<Sprite>,
        behavior_types: &[BehaviorType],
        should_run: bool,
        now: f64,
    ) {
        sprite.for_each(|sprite| {
            sprite
                .behaviors
                .iter_mut()
                .filter(|behavior| behavior_types.contains(&behavior.name()))
                .for_each(|behavior| behavior.toggle(should_run, now));
        });
    }
}
