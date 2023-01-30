use derives::{derive_behavior_fields, BaseBehavior};
use web_sys::CanvasRenderingContext2d;

use super::base::Behavior;
use crate::model::{BehaviorType, Position};
use crate::painter::Painter;
use crate::sprite::{Sprite, SpriteMutation};

#[derive_behavior_fields("")]
#[derive(BaseBehavior, Default)]
pub struct Cycle {
    name: BehaviorType,
}

impl Cycle {
    pub fn new() -> Cycle {
        Cycle {
            name: BehaviorType::Cycle,
            ..Default::default()
        }
    }
}

impl Behavior for Cycle {
    fn name(&self) -> BehaviorType {
        BehaviorType::Cycle
    }

    fn execute(
        &mut self,
        sprite: &Sprite,
        now: f64,
        _last_frame: f64,
        mouse: &Position,
        context: &CanvasRenderingContext2d,
    ) -> Option<SpriteMutation> {
        None
    }
}
