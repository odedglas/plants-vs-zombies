mod base;
mod click;
mod hover;

use std::borrow::{Borrow, BorrowMut};
use std::rc::Rc;
use std::slice::Iter;

pub use base::Behavior;
pub use click::Click;
pub use hover::Hover;
use web_sys::CanvasRenderingContext2d;

use crate::log;
use crate::model::{BehaviorData, BehaviorType, Position};
use crate::sprite::{Sprite, SpriteMutation};
use crate::timers::GameTime;

pub struct BehaviorManager;

impl BehaviorManager {
    pub fn create(data: &BehaviorData) -> Box<dyn Behavior> {
        match data.name.trim() {
            "Click" => Box::new(Click::new()),
            _ => Box::new(Hover::new()),
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
}
