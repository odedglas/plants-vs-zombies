use crate::sprite::behavior::collision::base::{CollisionHandler, DelayedMutation};
use crate::sprite::SpriteMutation;

pub struct BulletCollisionHandler;

impl CollisionHandler for BulletCollisionHandler {
    fn on_attack(&mut self) -> SpriteMutation {
        SpriteMutation::new().swap(0).mute(true)
    }

    fn on_after_attack(&mut self) -> DelayedMutation {
        (Some(SpriteMutation::new().hide(true)), 50.0)
    }
}
