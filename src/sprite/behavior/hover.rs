use derives::{derive_behavior_fields, BaseBehavior};
use web_sys::CanvasRenderingContext2d;

use super::base::Behavior;
use crate::model::{BehaviorType, Position};
use crate::painter::Painter;
use crate::sprite::{Sprite, SpriteMutation};

#[derive_behavior_fields("")]
#[derive(BaseBehavior, Default)]
pub struct Hover {
}

impl Hover {
    pub fn new() -> Hover {
        Hover {
            ..Default::default()
        }
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
