use crate::model::Callback;
use crate::sprite::{CollisionState, Sprite, SpriteMutation};

pub type DelayedMutation = (Option<SpriteMutation>, f64);

pub trait CollisionHandler {
    fn tick(&mut self) -> Option<SpriteMutation> {
        None
    }

    fn on_attack(&mut self) -> SpriteMutation {
        SpriteMutation::new()
    }

    fn on_after_attack(&mut self) -> DelayedMutation {
        (None, 0.0)
    }

    fn on_hit(&mut self, damage: f64) -> SpriteMutation {
        SpriteMutation::new().damage(damage)
    }

    fn on_after_hit(&mut self) -> DelayedMutation {
        (None, 0.0)
    }

    fn on_die(&mut self, _damage: f64) -> SpriteMutation {
        SpriteMutation::new().hide(true)
    }

    fn on_collision_state_change(
        &mut self,
        _sprite: &Sprite,
        _state: &CollisionState,
        _prev_state: &CollisionState,
    ) -> Option<SpriteMutation> {
        None
    }

    fn get_interaction_callback(&mut self) -> Option<Callback> {
        None
    }
}
