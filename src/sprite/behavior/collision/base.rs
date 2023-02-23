use crate::sprite::SpriteMutation;

pub type DelayedMutation = (Option<SpriteMutation>, f64);

pub trait CollisionHandler {
    fn on_attack(&self) -> SpriteMutation {
        SpriteMutation::new()
    }

    fn on_after_attack(&self) -> DelayedMutation {
        (Some(SpriteMutation::new()), 0.0)
    }

    fn on_hit(&self, damage: f64) -> SpriteMutation {
        SpriteMutation::new().damage(damage)
    }

    fn on_after_hit(&self) -> DelayedMutation {
        (Some(SpriteMutation::new()), 0.0)
    }
}

pub struct BulletCollisionHandler;

impl CollisionHandler for BulletCollisionHandler {
    fn on_attack(&self) -> SpriteMutation {
        SpriteMutation::new().swap(0).mute()
    }

    fn on_after_attack(&self) -> DelayedMutation {
        (Some(SpriteMutation::new().hide(true)), 50.0)
    }
}

pub struct PlantCollisionHandler;

impl CollisionHandler for PlantCollisionHandler {}

pub struct ZombieCollisionHandler;

impl CollisionHandler for ZombieCollisionHandler {
    fn on_hit(&self, damage: f64) -> SpriteMutation {
        SpriteMutation::new().damage(damage).alpha(0.5)
    }

    fn on_after_hit(&self) -> DelayedMutation {
        (Some(SpriteMutation::new().alpha(1.0)), 50.0)
    }
}
