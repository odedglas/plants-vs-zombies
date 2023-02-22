use derives::{derive_behavior_fields, BaseBehavior};
use web_sys::CanvasRenderingContext2d;

use super::base::Behavior;
use crate::log;
use crate::model::{BehaviorType, CollisionMargin, Position};
use crate::sprite::{Sprite, SpriteMutation};
use crate::timers::Timer;

#[derive(Debug, PartialEq)]
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
    pub last_collided: f64,

    internal_timer: Timer,
    delayed_mutation: Option<SpriteMutation>,
}

impl Collision {
    pub fn new(margin: CollisionMargin) -> Collision {
        Collision {
            margin,
            name: BehaviorType::Collision,
            state: CollisionState::None,
            delayed_mutation: None,
            internal_timer: Timer::new(10000.0),
            ..Default::default()
        }
    }

    fn set_delayed_mutation(&mut self, delay: f64, delayed_mutation: SpriteMutation) {
        self.internal_timer.set_elapsed(delay);
        self.internal_timer.start();

        self.delayed_mutation = Some(delayed_mutation);
    }
}

impl Behavior for Collision {
    fn name(&self) -> BehaviorType {
        BehaviorType::Collision
    }

    /// Collision state is managed by the BattleManager,
    /// this behavior just react into it's calculation instead of calculating its own.
    fn execute(
        &mut self,
        sprite: &Sprite,
        _now: f64,
        _last_frame: f64,
        _mouse: &Position,
        _context: &CanvasRenderingContext2d,
    ) -> Option<SpriteMutation> {
        let mut mutation: Option<SpriteMutation> = None;
        let window_time = self.internal_timer.get_current_time(); // TODO Refactor start timer to work with `now`.

        if self.delayed_mutation.is_some() && self.internal_timer.expired(window_time) {
            let mutation = self.delayed_mutation.clone();
            self.delayed_mutation = None;

            return mutation;
        }

        match self.state {
            CollisionState::None => {}
            CollisionState::Attacking => {
                if !self.delayed_mutation.is_some() {
                    // TODO - Extract into a specific collision handle
                    self.set_delayed_mutation(50.0, SpriteMutation::new().hide(true));
                }

                mutation = Some(SpriteMutation::new().swap(0).mute());
            }
            CollisionState::TakingDamage(damage) => {
                if damage > 0.0 {
                    log!(
                        "Taking damage by {}  of / {}",
                        sprite.attack_state.life,
                        damage
                    );

                    if !self.delayed_mutation.is_some() {
                        self.set_delayed_mutation(50.0, SpriteMutation::new().alpha(1.0));
                    }

                    mutation = Some(SpriteMutation::new().damage(damage).alpha(0.5))
                }
            }
        }

        mutation
    }
}
