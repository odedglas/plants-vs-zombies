use derives::{derive_behavior_fields, BaseBehavior};
use js_sys::Math::abs;
use web_sys::CanvasRenderingContext2d;

use super::base::Behavior;
use crate::log;
use crate::model::{BehaviorType, Callback, Position};
use crate::sprite::{Sprite, SpriteMutation};

#[derive_behavior_fields("")]
#[derive(BaseBehavior, Default)]
pub struct Drag {
    pub name: BehaviorType,
    anchor: Option<Position>,
    original_position: Position,
}

impl Drag {
    pub fn new(callback: Callback) -> Drag {
        Drag {
            name: BehaviorType::Drag,
            ..Default::default()
        }
    }

    pub fn calculate_mouse_offset(&mut self, sprite: &Sprite, mouse: &Position) -> Position {
        let current_anchor = match self.anchor {
            None => *mouse,
            Some(anchor) => anchor,
        };

        // Get offset between current mouse and anchor
        let offset_left = mouse.left - current_anchor.left;
        let offset_top = mouse.top - current_anchor.top;

        // Resets anchor
        self.anchor = Some(*mouse);

        Position::new(
            sprite.position.top + offset_top,
            sprite.position.left + offset_left,
        )
    }
}

impl Behavior for Drag {
    fn name(&self) -> BehaviorType {
        BehaviorType::Drag
    }

    fn execute(
        &mut self,
        sprite: &Sprite,
        now: f64,
        _last_frame: f64,
        mouse: &Position,
        context: &CanvasRenderingContext2d,
    ) -> Option<SpriteMutation> {
        // TODO - Condition with dragged_sprite_id

        // Mouse is Top / Left, Decrease the delta of the location on the sprite.
        let drag_offset = self.calculate_mouse_offset(sprite, mouse);

        Some(SpriteMutation::new().position(drag_offset))
    }
}
