use crate::model::Callback;
use crate::sprite::behavior::collision::base::CollisionHandler;
use crate::sprite::{CollisionState, Sprite, SpriteMutation};

pub struct LawnCleanerCollisionHandler {
    collided: bool
}

impl LawnCleanerCollisionHandler {
    pub fn new() -> Self {
        LawnCleanerCollisionHandler { collided: false }
    }
}

impl CollisionHandler for LawnCleanerCollisionHandler {
    fn on_collision_state_change(
        &mut self,
        _sprite: &Sprite,
        state: &CollisionState,
        _prev_state: &CollisionState,
    ) -> Option<SpriteMutation> {
        if state == &CollisionState::Attacking {
            self.collided = true;
            return Some(SpriteMutation::new().toggle_walking(true));
        }

        None
    }

    fn get_interaction_callback(&mut self) -> Option<Callback> {
        if self.collided {
            self.collided = false;
            return Some(Callback::LawnCleanerLost);
        }

        None
    }
}
