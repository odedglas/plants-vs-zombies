use web_sys::CanvasRenderingContext2d;

use super::base::{Behavior, BehaviorState};
use crate::model::{BehaviorType, Callback, GameInteraction, Position};
use crate::painter::Painter;
use crate::sprite::{Sprite, SpriteMutation};

pub struct Click {
    name: BehaviorType,
    running: bool,
    interaction_active: bool,
    callback: Callback,
}

impl Click {
    pub fn new(callback: Callback) -> Click {
        Click {
            callback,
            name: BehaviorType::Click,
            running: false,
            interaction_active: false,
        }
    }
}

impl BehaviorState for Click {
    fn start(&mut self, _now: f64) {
        self.running = true;
    }

    fn stop(&mut self, _now: f64) {
        self.running = false;
    }

    fn is_running(&self) -> bool {
        self.running
    }
}

impl Behavior for Click {
    fn name(&self) -> BehaviorType {
        BehaviorType::Click
    }

    fn get_interaction(&self) -> Option<GameInteraction> {
        if self.interaction_active {
            return Some(GameInteraction::SpriteClick(self.callback));
        }

        None
    }

    fn clean_interaction(&mut self) {
        self.interaction_active = false;
    }

    fn execute(
        &mut self,
        sprite: &Sprite,
        now: f64,
        _last_frame: f64,
        mouse: &Position,
        context: &CanvasRenderingContext2d,
    ) -> Option<SpriteMutation> {
        self.stop(now);

        let clicked = Painter::in_path(&sprite.outlines, mouse, context);

        if clicked {
            self.interaction_active = true;
        }

        None
    }
}
