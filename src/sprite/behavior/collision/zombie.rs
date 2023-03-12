use crate::model::Callback;
use crate::sprite::behavior::collision::base::{CollisionHandler, DelayedMutation};
use crate::sprite::{CollisionState, Sprite, SpriteMutation};
use crate::timers::Timer;
use crate::web_utils::window_time;

#[derive(Copy, Clone, PartialEq)]
enum ZombieState {
    Stale = 0,
    ArmoredWalk,
    ArmoredAttack,
    Walk,
    Attack,
    Die,
}

impl ZombieState {
    pub fn index(&self) -> usize {
        *self as usize
    }
}

pub struct ZombieCollisionHandler {
    attack_timer: Timer,
    zombie_state: ZombieState,
    lost_head: bool,
}

impl ZombieCollisionHandler {
    pub fn new() -> Self {
        ZombieCollisionHandler {
            attack_timer: Timer::new(2000.0),
            zombie_state: ZombieState::ArmoredWalk,
            lost_head: false,
        }
    }

    fn get_zombie_state(&mut self, state: &CollisionState, life: f64) -> ZombieState {
        match state {
            CollisionState::None | CollisionState::ApplyEffect(_) => match life <= 100.0 {
                true => ZombieState::Walk,
                false => ZombieState::ArmoredWalk,
            },
            CollisionState::Attacking => match life <= 100.0 {
                true => ZombieState::Attack,
                false => ZombieState::ArmoredAttack,
            },
            CollisionState::TakingDamage(_) => match life <= 0.0 {
                true => ZombieState::Die,
                false => self.zombie_state,
            },
        }
    }

    fn get_swap_index(&mut self) -> i32 {
        (self.zombie_state.index() - 1) as i32
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

        SpriteMutation::new().mute(true).swap(self.get_swap_index())
    }

    fn on_hit(&mut self, damage: f64) -> SpriteMutation {
        SpriteMutation::new()
            .take_damage(damage)
            .alpha(0.5)
            .swap(self.get_swap_index())
    }

    fn on_after_hit(&mut self) -> DelayedMutation {
        (Some(SpriteMutation::new().alpha(1.0)), 50.0)
    }

    fn on_die(&mut self, damage: f64) -> SpriteMutation {
        self.zombie_state = ZombieState::Die;

        SpriteMutation::new()
            .take_damage(damage)
            .alpha(0.9) // TODO - Can be replaced with fadeout effect
            .mute(true)
            .swap(self.get_swap_index())
            .stop_animate()
    }

    fn on_collision_state_change(
        &mut self,
        sprite: &Sprite,
        state: &CollisionState,
        prev_state: &CollisionState,
    ) -> Option<SpriteMutation> {
        let life = sprite.attack_state.life;
        let prev_zombie_state = self.zombie_state;
        self.zombie_state = self.get_zombie_state(state, life);

        // Once Zombie Stop attacking.
        if state == &CollisionState::None
            && prev_state == &CollisionState::Attacking
            && self.attack_timer.running
        {
            self.attack_timer.stop(None);
            return Some(
                SpriteMutation::new()
                    .mute(false)
                    .swap(self.get_swap_index()),
            );
        }

        // Internal Zombie state changes.
        if prev_zombie_state != self.zombie_state {
            return Some(SpriteMutation::new().swap(self.get_swap_index()));
        }

        None
    }

    fn get_interaction_callback(&mut self) -> Option<Callback> {
        if self.zombie_state == ZombieState::Die && !self.lost_head {
            self.lost_head = true;
            return Some(Callback::OnZombieDeath);
        }

        None
    }
}
