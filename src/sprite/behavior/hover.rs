use web_sys::CanvasRenderingContext2d;

use super::base::{Behavior, BehaviorState};
use crate::log;
use crate::model::{BehaviorType, Position};
use crate::sprite::SpriteMutation;

pub struct Hover {
    name: BehaviorType,
    running: bool,
    id: String,
}

impl Hover {
    pub fn new(id: String) -> Hover {
        Hover {
            id,
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
    fn id(&self) -> &String {
        &self.id
    }

    fn name(&self) -> BehaviorType {
        BehaviorType::Hover
    }

    fn execute(
        &mut self,
        now: f64,
        _last_frame: f64,
        mouse: &Position,
        _context: &CanvasRenderingContext2d,
    ) -> Option<SpriteMutation> {
        log!("Execute Hover aciton! {} / {:?}", now, mouse);

        Some(SpriteMutation::new(
            Some(Position {
                left: 0.0,
                top: 1.0,
            }),
            None,
            None,
        ))
    }
}
