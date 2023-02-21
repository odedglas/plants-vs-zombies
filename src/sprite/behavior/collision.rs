use derives::{derive_behavior_fields, BaseBehavior};
use web_sys::CanvasRenderingContext2d;

use super::base::Behavior;
use crate::log;
use crate::model::{BehaviorType, Callback, CollisionMargin, GameInteraction, Position};
use crate::painter::Painter;
use crate::sprite::{Sprite, SpriteMutation};

#[derive(Debug)]
pub enum CollisionState {
    None,
    Attacking,
    TakingDamage(f64),
}

impl Default for CollisionState {
    fn default() -> Self {
        CollisionState::None
    }
}

#[derive_behavior_fields("")]
#[derive(BaseBehavior, Default)]
pub struct Collision {
    pub name: BehaviorType,
    pub margin: CollisionMargin,
    pub state: CollisionState,
}

impl Collision {
    pub fn new(margin: CollisionMargin) -> Collision {
        Collision {
            margin,
            name: BehaviorType::Collision,
            state: CollisionState::None,
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
        let mut mutation: Option<SpriteMutation> = None;
        // Collision state is managed by the BattleManager, this behavior just react into it's calculation instead of calculating its own.
        // TODO - Handle by CollisionType ?
        // This should handle the `collided` flag respecting the type

        match self.state {
            CollisionState::None => {}
            CollisionState::Attacking => {
                log!("CollisionAttack by {} ", sprite.id);
                mutation = Some(SpriteMutation::new().hide());
            }
            CollisionState::TakingDamage(damage) => {
                log!(
                    "Taking damage by {}  of / {}",
                    sprite.attack_state.life,
                    damage
                );
                mutation = Some(SpriteMutation::new().damage(damage))
            }
        }

        self.state = CollisionState::None;

        mutation
    }
}
