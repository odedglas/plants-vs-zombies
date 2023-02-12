use derives::{derive_behavior_fields, BaseBehavior};
use web_sys::CanvasRenderingContext2d;

use super::base::Behavior;
use crate::model::{BehaviorType, Callback, Position};
use crate::sprite::{Sprite, SpriteMutation};

#[derive_behavior_fields("")]
#[derive(BaseBehavior, Default)]
pub struct Drag {
    pub name: BehaviorType,
}

impl Drag {
    pub fn new(callback: Callback) -> Drag {
        Drag {
            name: BehaviorType::Drag,
            ..Default::default()
        }
    }
}

impl Behavior for Drag {
    fn name(&self) -> BehaviorType {
        BehaviorType::Drag
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
