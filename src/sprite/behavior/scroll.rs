use derives::{derive_behavior_fields, BaseBehavior};
use web_sys::CanvasRenderingContext2d;

use super::base::Behavior;
use crate::log;
use crate::model::{BehaviorType, Callback, GameInteraction, Position};
use crate::sprite::{Sprite, SpriteMutation};

const SCROLL_ADDITION: f64 = 8.5;

#[derive_behavior_fields("")]
#[derive(BaseBehavior, Default)]
pub struct Scroll {
    name: BehaviorType,
    callback: Callback,
    distance: f64,
    duration: f64,
    last_tick: f64,
    scrolled_distance: f64,
}

impl Scroll {
    pub fn new(distance: f64, duration: f64, callback: Callback) -> Scroll {
        Scroll {
            callback,
            duration,
            distance,
            name: BehaviorType::Click,
            ..Default::default()
        }
    }
}

impl Behavior for Scroll {
    fn name(&self) -> BehaviorType {
        BehaviorType::Scroll
    }

    fn get_interaction(&self) -> Option<GameInteraction> {
        if self.interaction_active {
            return Some(GameInteraction::SpriteClick(self.callback));
        }

        None
    }

    fn execute(
        &mut self,
        sprite: &Sprite,
        now: f64,
        _last_frame: f64,
        _mouse: &Position,
        _context: &CanvasRenderingContext2d,
    ) -> Option<SpriteMutation> {
        let finished = self.scrolled_distance >= self.distance;
        let should_scroll = now - self.last_tick >= self.duration;

        if finished {
            self.stop(now);
            self.interaction_active = true;

            return None;
        }

        // TODO - Needs to consider also directions e.g Starts with right, a ends with a left once re-toggled
        // State will be preserved so we can tell if we directed.
        if should_scroll {
            self.last_tick = now;
            self.scrolled_distance += SCROLL_ADDITION;
            let current_offset = &sprite.drawing_state.offset;

            return Some(SpriteMutation::new().offset(Position::new(
                current_offset.top,
                current_offset.left + SCROLL_ADDITION,
            )));
        }

        None
    }
}
