use derives::{derive_behavior_fields, BaseBehavior};
use web_sys::CanvasRenderingContext2d;

use super::base::Behavior;
use crate::model::{BehaviorType, Callback, GameInteraction, Position};
use crate::painter::Painter;
use crate::sprite::{Sprite, SpriteMutation};

#[derive_behavior_fields("")]
#[derive(BaseBehavior, Default)]
pub struct Drag {
    pub name: BehaviorType,
    anchor: Option<Position>,
    callback: Callback,
    dragging: bool,
}

impl Drag {
    pub fn new(callback: Callback) -> Drag {
        Drag {
            name: BehaviorType::Drag,
            callback,
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

    fn on_stop(&mut self, _now: f64) {
        self.dragging = false;
        self.anchor = None;
        self.interaction_active = true;
    }

    fn get_interaction(&self) -> Option<GameInteraction> {
        if self.interaction_active {
            return Some(GameInteraction::SpriteClick(
                self.callback,
                self.sprite_id.clone(),
            ));
        }

        None
    }

    fn execute(
        &mut self,
        sprite: &Sprite,
        _now: f64,
        _last_frame: f64,
        mouse: &Position,
        context: &CanvasRenderingContext2d,
    ) -> Option<SpriteMutation> {
        let is_dragging = self.dragging;

        // Hovers initialise the drag action, and later we flag using the is_dragging for better control (Mouse up triggers the on_stop)
        let hovering = !is_dragging && Painter::in_path(&sprite.outlines, mouse, context);

        if is_dragging || hovering {
            // Mouse is Top / Left, Decrease the delta of the location on the sprite.
            let drag_offset = self.calculate_mouse_offset(sprite, mouse);

            self.dragging = true;

            return Some(SpriteMutation::new().position(drag_offset));
        }

        None
    }
}
