use derives::{derive_behavior_fields, BaseBehavior};
use web_sys::CanvasRenderingContext2d;

use super::base::Behavior;
use crate::model::{BehaviorType, Callback, GameInteraction, Position};
use crate::sprite::{Sprite, SpriteMutation};

#[derive_behavior_fields("")]
#[derive(BaseBehavior, Default)]
pub struct Animate {
    last_tick: f64,
    rate: f64,
    finished_cycles: usize,
    max_cycles: usize,
    callback: Option<Callback>,
    callback_delay: f64,
}

impl Animate {
    pub fn new(
        rate: f64,
        callback: Option<Callback>,
        callback_delay: Option<f64>,
        max_cycles: Option<usize>,
    ) -> Animate {
        Animate {
            rate,
            callback,
            callback_delay: callback_delay.unwrap_or(1000.0),
            max_cycles: max_cycles.unwrap_or(1),
            ..Default::default()
        }
    }

    pub fn set_max_cycles(&mut self, max_cycles: usize) {
        self.finished_cycles = 0;
        self.max_cycles = max_cycles;
    }
}

impl Behavior for Animate {
    fn name(&self) -> BehaviorType {
        BehaviorType::Animate
    }

    fn get_interaction(&self) -> Option<GameInteraction> {
        if !self.interaction_active || self.callback.is_none() {
            return None;
        }

        Some(GameInteraction::AnimationCallback(
            self.callback.unwrap(),
            self.sprite_id.clone(),
        ))
    }

    fn on_start(&mut self, now: f64) {
        self.last_tick = now;
    }

    fn execute(
        &mut self,
        sprite: &Sprite,
        now: f64,
        _last_frame: f64,
        _mouse: &Position,
        _context: &CanvasRenderingContext2d,
    ) -> Option<SpriteMutation> {
        let infinite = self.max_cycles == 0;

        let finished = !infinite && self.finished_cycles == self.max_cycles;
        let should_animate = now - self.last_tick >= self.rate;

        if finished {
            let execute_callback = now - self.last_tick > self.callback_delay;

            // Checking if should trigger animation callback and finish animation
            if execute_callback {
                self.stop(now);
                self.interaction_active = true;
                return Some(SpriteMutation::new().hide(true));
            }

            return None;
        }

        if should_animate {
            if sprite.drawing_state.in_last_cell() {
                self.finished_cycles += 1;
            }

            // Animate Sprite cells as long as we didn't finish the desired "max_cycles" animation amount.
            if infinite || self.finished_cycles != self.max_cycles {
                self.last_tick = now;
                return Some(SpriteMutation::new().cycle());
            }
        }

        None
    }
}
