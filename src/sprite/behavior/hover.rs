use web_sys::CanvasRenderingContext2d;

use super::base::{Behavior, BehaviorState};
use crate::model::{BehaviorType, Position};
use crate::painter::Painter;
use crate::sprite::{Sprite, SpriteMutation};

pub struct Hover {
    name: BehaviorType,
    running: bool,
}

impl Hover {
    pub fn new() -> Hover {
        Hover {
            name: BehaviorType::Hover,
            running: false,
        }
    }
}

impl BehaviorState for Hover {
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

impl Behavior for Hover {
    fn name(&self) -> BehaviorType {
        BehaviorType::Hover
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

        let hovered = Painter::in_path(&sprite.outlines, mouse, context);

        Some(SpriteMutation::new().hovered(hovered))
    }
}
