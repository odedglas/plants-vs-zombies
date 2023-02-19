use derives::{derive_behavior_fields, BaseBehavior};
use web_sys::CanvasRenderingContext2d;

use super::base::Behavior;
use crate::log;
use crate::model::{BehaviorType, Callback, CollisionMargin, GameInteraction, Position};
use crate::painter::Painter;
use crate::sprite::{Sprite, SpriteMutation};

#[derive_behavior_fields("")]
#[derive(BaseBehavior, Default)]
pub struct Collision {
    pub name: BehaviorType,
    pub margin: CollisionMargin,
}

impl Collision {
    pub fn new(margin: CollisionMargin) -> Collision {
        log!("Creating Collision behavior withs {:?}", margin);
        Collision {
            margin,
            name: BehaviorType::Collision,
            ..Default::default()
        }
    }
}

impl Behavior for Collision {
    fn name(&self) -> BehaviorType {
        BehaviorType::Collision
    }

    fn execute(
        &mut self,
        sprite: &Sprite,
        now: f64,
        _last_frame: f64,
        mouse: &Position,
        context: &CanvasRenderingContext2d,
    ) -> Option<SpriteMutation> {
        // TODO - Handle by CollisionType ?
        // This should handle the `collided` flag respecting the type

        None
    }
}
