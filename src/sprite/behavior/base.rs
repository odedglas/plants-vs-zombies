use std::any::Any;

use web_sys::CanvasRenderingContext2d;

use crate::model::{BehaviorType, Callback, GameInteraction, Position};
use crate::sprite::{Sprite, SpriteMutation};

pub trait BehaviorState {
    fn start(&mut self, now: f64);

    fn stop(&mut self, now: f64);

    fn is_running(&self) -> bool;

    fn toggle(&mut self, run: bool, now: f64) {
        match run {
            true => self.start(now),
            false => self.stop(now),
        }
    }

    fn clean_interaction(&mut self);

    fn set_sprite_id(&mut self, sprite_id: String);

    fn as_any(&mut self) -> &mut dyn Any;
}

pub trait Behavior: BehaviorState {
    fn name(&self) -> BehaviorType;

    fn get_interaction(&self) -> Option<GameInteraction> {
        return None;
    }

    fn animation_rate(&mut self, now: f64, last_frame: f64) -> f64 {
        (now - last_frame) / 1000.0
    }

    fn execute(
        &mut self,
        sprite: &Sprite,
        now: f64,
        last_frame: f64,
        mouse: &Position,
        context: &CanvasRenderingContext2d,
    ) -> Option<SpriteMutation>;
}
