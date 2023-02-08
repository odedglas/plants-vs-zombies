use derives::{derive_behavior_fields, BaseBehavior};
use web_sys::CanvasRenderingContext2d;

use super::base::Behavior;
use crate::log;
use crate::model::{BehaviorType, Position, Velocity};
use crate::sprite::{Sprite, SpriteMutation};

#[derive_behavior_fields("")]
#[derive(BaseBehavior, Default)]
pub struct Walk {
    name: BehaviorType,
    velocity: Velocity,
    rate: f64,
    max_distance: f64,
    walked_distance: f64,
}

impl Walk {
    pub fn new(rate: f64, distance: f64, velocity: Velocity) -> Walk {
        Walk {
            name: BehaviorType::Walk,
            rate,
            velocity,
            max_distance: distance,
            ..Default::default()
        }
    }
}

impl Behavior for Walk {
    fn name(&self) -> BehaviorType {
        BehaviorType::Walk
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

        log!("Running Walk Behavior {:?}", self.velocity);

        None
    }
}
