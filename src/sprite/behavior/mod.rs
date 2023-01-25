mod base;
mod click;
mod hover;

pub use base::Behavior;
pub use click::Click;
pub use hover::Hover;
use web_sys::CanvasRenderingContext2d;

use crate::log;
use crate::model::Position;
use crate::sprite::{Sprite, SpriteMutation};

pub struct BehaviorManager;

impl BehaviorManager {
    pub fn create(id: &str, name: &str) -> Box<dyn Behavior> {
        let behavior_id = String::from(id);

        match name {
            "Click" => Box::new(Click::new(behavior_id)),
            _ => Box::new(Hover::new(behavior_id)),
        }
    }

    pub fn run_sprite_behaviours(
        sprite: &mut Sprite,
        now: f64,
        last_frame: f64,
        position: &Position,
        context: &CanvasRenderingContext2d,
    ) -> Vec<SpriteMutation> {
        let behaviors: &mut Vec<Box<dyn Behavior>> = sprite.behaviors.as_mut();

        behaviors
            .iter_mut()
            .filter(|behavior| behavior.is_running())
            .map(|behavior| behavior.execute(now, last_frame, position, context))
            .filter_map(|mutation| mutation)
            .collect()
    }
}
