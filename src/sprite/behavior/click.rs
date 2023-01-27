use web_sys::CanvasRenderingContext2d;

use super::base::{Behavior, BehaviorState};
use crate::log;
use crate::model::{BehaviorType, Position};
use crate::painter::Painter;
use crate::sprite::{Sprite, SpriteMutation};

pub struct Click {
    name: BehaviorType,
    running: bool,
}

impl Click {
    pub fn new() -> Click {
        Click {
            name: BehaviorType::Click,
            running: false,
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

    fn execute(
        &mut self,
        sprite: &Sprite,
        now: f64,
        _last_frame: f64,
        mouse: &Position,
        context: &CanvasRenderingContext2d,
    ) -> Option<SpriteMutation> {
        self.stop(now);
        log!("Execute Click action! {} / {:?}", now, mouse);

        let clicked = Painter::in_path(&sprite.outlines, mouse, context);

        Some(SpriteMutation::new(None, None, Some(clicked)))
    }
}
