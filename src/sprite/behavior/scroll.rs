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
    rate: f64,
    scrolled_distance: f64,
    direction: i32,
}

impl Scroll {
    pub fn new(distance: f64, rate: f64, callback: Callback) -> Scroll {
        Scroll {
            callback,
            rate,
            distance,
            direction: 1,
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
            return Some(GameInteraction::SpriteClick(
                self.callback,
                self.sprite_id.clone(),
            ));
        }

        None
    }

    fn reverse(&mut self, now: f64, callback: Callback) {
        self.direction *= -1;
        self.scrolled_distance = 0.0;
        self.callback = callback;

        if !self.is_running() {
            self.start(now);
        }
    }

    fn execute(
        &mut self,
        sprite: &Sprite,
        now: f64,
        last_frame: f64,
        _mouse: &Position,
        _context: &CanvasRenderingContext2d,
    ) -> Option<SpriteMutation> {
        let finished = self.scrolled_distance.abs() >= self.distance;

        if finished {
            self.stop(now);
            self.interaction_active = true;

            return None;
        }

        let addition = self.rate * self.animation_rate(now, last_frame) * self.direction as f64;

        self.scrolled_distance += addition;

        let current_offset = &sprite.drawing_state.offset;

        return Some(SpriteMutation::new().offset(Position::new(
            current_offset.top,
            current_offset.left + addition,
        )));
    }
}
