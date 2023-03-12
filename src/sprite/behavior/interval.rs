use derives::{derive_behavior_fields, BaseBehavior};
use web_sys::CanvasRenderingContext2d;

use super::base::Behavior;
use crate::model::{BehaviorType, Callback, GameInteraction, Position};
use crate::sprite::{Sprite, SpriteMutation};
use crate::timers::Timer;

#[derive_behavior_fields("")]
#[derive(BaseBehavior, Default)]
pub struct Interval {
    pub callback: Option<Callback>,
    pub interval: f64,
    timer: Timer,
}

impl Interval {
    pub fn new(interval: f64, callback: Option<Callback>) -> Interval {
        Interval {
            callback,
            interval,
            timer: Timer::new(interval),
            ..Default::default()
        }
    }
}

impl Behavior for Interval {
    fn name(&self) -> BehaviorType {
        BehaviorType::Interval
    }

    fn on_stop(&mut self, now: f64) {
        self.timer.reset(Some(now))
    }

    fn on_start(&mut self, _now: f64) {
        self.timer.start();
    }

    fn get_interaction(&self) -> Option<GameInteraction> {
        if self.interaction_active {
            return Some(GameInteraction::SpriteClick(
                self.callback.unwrap(),
                self.sprite_id.clone(),
            ));
        }

        None
    }

    fn execute(
        &mut self,
        _sprite: &Sprite,
        now: f64,
        _last_frame: f64,
        _mouse: &Position,
        _context: &CanvasRenderingContext2d,
    ) -> Option<SpriteMutation> {
        if self.timer.expired(now) {
            match self.callback {
                None => {}
                Some(_) => self.interaction_active = true,
            }

            self.timer.reset(Some(now));
        }

        None
    }
}
