use crate::sprite::{CollisionState, SpriteMutation};
use crate::timers::Timer;
use crate::web_utils::window_time;

pub type DelayedMutation = (Option<SpriteMutation>, f64);

pub trait CollisionHandler {
    fn tick(&mut self) -> Option<SpriteMutation> {
        None
    }

    fn on_attack(&mut self) -> SpriteMutation {
        SpriteMutation::new()
    }

    fn on_after_attack(&mut self) -> DelayedMutation {
        (Some(SpriteMutation::new()), 0.0)
    }

    fn on_hit(&mut self, damage: f64) -> SpriteMutation {
        SpriteMutation::new().damage(damage)
    }

    fn on_after_hit(&mut self) -> DelayedMutation {
        (Some(SpriteMutation::new()), 0.0)
    }

    fn on_collision_state_change(
        &mut self,
        _state: &CollisionState,
        _prev_state: &CollisionState,
    ) -> Option<SpriteMutation> {
        None
    }
}

pub struct BulletCollisionHandler;

impl CollisionHandler for BulletCollisionHandler {
    fn on_attack(&mut self) -> SpriteMutation {
        SpriteMutation::new().swap(0).mute(true)
    }

    fn on_after_attack(&mut self) -> DelayedMutation {
        (Some(SpriteMutation::new().hide(true)), 50.0)
    }
}

pub struct PlantCollisionHandler;

impl CollisionHandler for PlantCollisionHandler {}

pub struct ZombieCollisionHandler {
    attack_timer: Timer,
}

impl ZombieCollisionHandler {
    pub fn new() -> Self {
        ZombieCollisionHandler {
            attack_timer: Timer::new(2000.0),
        }
    }
}

impl CollisionHandler for ZombieCollisionHandler {
    fn tick(&mut self) -> Option<SpriteMutation> {
        if self.attack_timer.expired(window_time()) {
            self.attack_timer.stop(None);
            return Some(SpriteMutation::new().mute(false));
        }

        None
    }

    fn on_attack(&mut self) -> SpriteMutation {
        if self.attack_timer.running {
            return SpriteMutation::new();
        }

        self.attack_timer.start();

        SpriteMutation::new().mute(true).swap(-1)
    }

    fn on_hit(&mut self, damage: f64) -> SpriteMutation {
        SpriteMutation::new().damage(damage).alpha(0.5)
    }

    fn on_after_hit(&mut self) -> DelayedMutation {
        (Some(SpriteMutation::new().alpha(1.0)), 50.0)
    }

    fn on_collision_state_change(
        &mut self,
        state: &CollisionState,
        prev_state: &CollisionState,
    ) -> Option<SpriteMutation> {
        if state == &CollisionState::None && prev_state == &CollisionState::Attacking {
            self.attack_timer.stop(None);
            return Some(SpriteMutation::new().mute(false).swap(0));
        }

        None
    }
}
