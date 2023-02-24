mod base;

use derives::{derive_behavior_fields, BaseBehavior};
use web_sys::CanvasRenderingContext2d;

use super::base::Behavior;
use crate::log;
use crate::model::{BehaviorType, CollisionMargin, Position, SpriteType};
use crate::sprite::behavior::collision::base::{
    BulletCollisionHandler, CollisionHandler, DelayedMutation, PlantCollisionHandler,
    ZombieCollisionHandler,
};
use crate::sprite::{Sprite, SpriteMutation};
use crate::timers::Timer;

#[derive(Debug, Clone, PartialEq)]
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
    pub prev_state: CollisionState,
    pub last_collided: f64,

    delayed_mutation_timer: Timer,
    handler: Option<Box<dyn CollisionHandler>>,
    delayed_mutation: Option<SpriteMutation>,
}

impl Collision {
    pub fn new(margin: CollisionMargin) -> Collision {
        Collision {
            margin,
            name: BehaviorType::Collision,
            state: CollisionState::None,
            prev_state: CollisionState::None,
            delayed_mutation: None,
            delayed_mutation_timer: Timer::new(10000.0),
            ..Default::default()
        }
    }

    fn set_collision_handler(&mut self, sprite_type: &SpriteType) {
        if self.handler.is_some() {
            return;
        }

        let handler: Box<dyn CollisionHandler> = match sprite_type {
            SpriteType::Zombie => Box::new(ZombieCollisionHandler::new()),
            SpriteType::Plant => Box::new(PlantCollisionHandler {}),
            SpriteType::Bullet => Box::new(BulletCollisionHandler {}),
            _ => {
                panic!("Cannot find Collision handler for {:?}", sprite_type)
            }
        };

        self.handler = Some(handler);
    }

    fn set_delayed_mutation(&mut self, delayed_mutation: DelayedMutation) {
        let (mutation, delay) = delayed_mutation;
        if !self.delayed_mutation.is_some() {
            match mutation {
                Some(mutation) => {
                    self.delayed_mutation_timer.set_elapsed(delay);
                    self.delayed_mutation_timer.start();

                    self.delayed_mutation = Some(mutation);
                }
                None => {}
            }
        }
    }
}

impl Behavior for Collision {
    fn name(&self) -> BehaviorType {
        BehaviorType::Collision
    }

    /// Collision state is managed by the BattleManager,
    /// this behavior is reactive to it's calculation instead of calculating its own collision state.
    /// General concept is each Sprite has its CollisionHandler which hooks into the `on_attack` / `on_hit` hooks
    /// Allowing the Sprite to affect it's own state.
    fn execute(
        &mut self,
        sprite: &Sprite,
        _now: f64,
        _last_frame: f64,
        _mouse: &Position,
        _context: &CanvasRenderingContext2d,
    ) -> Option<SpriteMutation> {
        let mut mutation: Option<SpriteMutation> = None;
        let current_timer_time = self.delayed_mutation_timer.get_current_time();

        // Ensures CollisionHandler is set
        self.set_collision_handler(&sprite.sprite_type);
        let collision_handler = self.handler.as_mut().unwrap();

        // Handles delayed mutation set by the handler if any
        if self.delayed_mutation.is_some() && self.delayed_mutation_timer.expired(current_timer_time)
        {
            let mutation = self.delayed_mutation.clone();
            self.delayed_mutation = None;

            return mutation;
        }

        // Handles handler `tick` phase mutation if set
        let tick_mutation = collision_handler.tick();
        if tick_mutation.is_some() {
            return tick_mutation;
        }

        if self.prev_state != self.state {
            let state_change_mutation =
                collision_handler.on_collision_state_change(&self.state, &self.prev_state);

            if state_change_mutation.is_some() {
                return state_change_mutation;
            }
        }

        // Handle Collision state hooks
        match self.state {
            CollisionState::None => {}
            CollisionState::Attacking => {
                mutation = Some(collision_handler.on_attack());

                let delayed_mutation = collision_handler.on_after_attack();
                self.set_delayed_mutation(delayed_mutation);
            }
            CollisionState::TakingDamage(damage) => {
                if damage > 0.0 {
                    mutation = Some(collision_handler.on_hit(damage));

                    let delayed_mutation = collision_handler.on_after_hit();
                    self.set_delayed_mutation(delayed_mutation);
                }
            }
        }

        self.prev_state = self.state.clone();

        mutation
    }
}