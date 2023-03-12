use crate::model::AttackEffect;
use crate::sprite::behavior::collision::base::{CollisionHandler, DelayedMutation};
use crate::sprite::behavior::collision::bullet::BulletState::Flying;
use crate::sprite::SpriteMutation;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BulletState {
    Hit,
    FireBullet,
    Flying,
}

impl BulletState {
    pub fn index(&self) -> i32 {
        *self as i32
    }
}

pub struct BulletCollisionHandler {
    state: BulletState,
}

impl BulletCollisionHandler {
    pub fn new() -> Self {
        BulletCollisionHandler { state: Flying }
    }
}

impl CollisionHandler for BulletCollisionHandler {
    fn on_attack(&mut self) -> SpriteMutation {
        self.state = BulletState::Hit;
        SpriteMutation::new().swap(self.state.index()).mute(true)
    }

    fn on_after_attack(&mut self) -> DelayedMutation {
        (Some(SpriteMutation::new().hide(true)), 50.0)
    }

    fn on_apply_effect(&mut self, effect: AttackEffect) -> SpriteMutation {
        if effect == AttackEffect::TurnIntoFireBullet && self.state != BulletState::FireBullet {
            self.state = BulletState::FireBullet;

            return SpriteMutation::new()
                .swap(self.state.index())
                .increase_damage(15.0);
        }

        SpriteMutation::new()
    }
}
