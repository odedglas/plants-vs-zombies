use derives::{derive_behavior_fields, BaseBehavior};
use web_sys::CanvasRenderingContext2d;

use super::base::Behavior;
use crate::model::{BehaviorType, Callback, GameInteraction, Position};
use crate::sprite::{Sprite, SpriteMutation};

#[derive_behavior_fields("")]
#[derive(BaseBehavior, Default)]
pub struct Animate {
    name: BehaviorType,
    last_tick: f64,
    duration: f64,
    finished_cycles: usize,
    max_cycles: usize,
    callback: Option<Callback>,
    callback_delay: f64,
}

impl Animate {
    pub fn new(
        duration: f64,
        callback: Option<Callback>,
        callback_delay: Option<f64>,
        max_cycles: Option<usize>,
    ) -> Animate {
        Animate {
            name: BehaviorType::Animate,
            duration,
            callback,
            callback_delay: callback_delay.unwrap_or(1000.0),
            max_cycles: max_cycles.unwrap_or(1),
            ..Default::default()
        }
    }
}

impl Behavior for Animate {
    fn name(&self) -> BehaviorType {
        BehaviorType::Animate
    }

    fn get_interaction(&self) -> Option<GameInteraction> {
        if !self.interaction_active || !self.callback.is_some() {
            return None;
        }

        return Some(GameInteraction::AnimationCallback(
            self.callback.unwrap(),
            self.sprite_id.clone(),
        ));
    }

    fn execute(
        &mut self,
        sprite: &Sprite,
        now: f64,
        _last_frame: f64,
        _mouse: &Position,
        _context: &CanvasRenderingContext2d,
    ) -> Option<SpriteMutation> {
        let finished = self.finished_cycles == self.max_cycles;
        let should_animate = now - self.last_tick >= self.duration;

        if finished {
            let execute_callback = now - self.last_tick > self.callback_delay;

            // Checking if should trigger animation callback and finish animation
            if execute_callback {
                self.stop(now);
                self.interaction_active = true;
            }

            return None;
        }

        if should_animate {
            if sprite.drawing_state.in_last_cell() {
                self.finished_cycles += 1;
            }

            // Animate Sprite cells as long as we didn't finish the desired "max_cycles" animation amount.
            if self.finished_cycles != self.max_cycles {
                self.last_tick = now;
                return Some(SpriteMutation::new().cycle());
            }
        }

        None
    }
}
