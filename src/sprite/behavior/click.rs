use derives::{derive_behavior_fields, BaseBehavior};
use web_sys::CanvasRenderingContext2d;

use super::base::Behavior;
use crate::model::{BehaviorType, Callback, GameInteraction, Position};
use crate::painter::Painter;
use crate::sprite::{Sprite, SpriteMutation};

#[derive_behavior_fields("")]
#[derive(BaseBehavior, Default)]
pub struct Click {
    name: BehaviorType,
    callback: Callback,
}

impl Click {
    pub fn new(callback: Callback) -> Click {
        Click {
            callback,
            name: BehaviorType::Click,
            ..Default::default()
        }
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
