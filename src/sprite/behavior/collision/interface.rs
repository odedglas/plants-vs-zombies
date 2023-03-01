use crate::sprite::behavior::collision::base::CollisionHandler;
use crate::sprite::{CollisionState, Sprite, SpriteMutation};

pub struct InterfaceCollisionHandler;

impl CollisionHandler for InterfaceCollisionHandler {
    fn on_collision_state_change(
        &mut self,
        _sprite: &Sprite,
        state: &CollisionState,
        prev_state: &CollisionState,
    ) -> Option<SpriteMutation> {
        if state != &CollisionState::None && prev_state == &CollisionState::None {
            return Some(SpriteMutation::new().toggle_walking(true));
        }

        None
    }
}
