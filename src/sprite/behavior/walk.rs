use derives::{derive_behavior_fields, BaseBehavior};
use web_sys::CanvasRenderingContext2d;

use super::base::Behavior;
use crate::board::Board;
use crate::log;
use crate::model::{BehaviorType, Callback, GameInteraction, Position, Velocity};
use crate::sprite::{Sprite, SpriteMutation};

#[derive_behavior_fields("")]
#[derive(BaseBehavior, Default)]
pub struct Walk {
    name: BehaviorType,
    pub velocity: Velocity,
    max_distance: f64,
    walked_distance: f64,
    callback: Option<Callback>
}

impl Walk {
    pub fn new(distance: f64, velocity: Velocity, callback: Option<Callback>) -> Walk {
        Walk {
            name: BehaviorType::Walk,
            velocity,
            callback,
            max_distance: distance,
            ..Default::default()
        }
    }

    fn calculate_offset(&mut self, animation_rate: f64) -> Position {
        Position::new(
            animation_rate * self.velocity.y,
            animation_rate * self.velocity.x,
        )
    }

    fn position_distance(&self, position: &Position) -> f64 {
        (position.left.abs().powf(2.0) + position.top.abs().powf(2.0)).sqrt()
    }
}

impl Behavior for Walk {
    fn name(&self) -> BehaviorType {
        BehaviorType::Walk
    }

    fn get_interaction(&self) -> Option<GameInteraction> {
        if self.interaction_active && self.callback.is_some() {
            return Some(GameInteraction::SpriteClick(
                self.callback.unwrap(),
                self.sprite_id.clone(),
            ));
        }

        None
    }

    fn execute(
        &mut self,
        sprite: &Sprite,
        now: f64,
        last_frame: f64,
        _mouse: &Position,
        _context: &CanvasRenderingContext2d,
    ) -> Option<SpriteMutation> {
        let finished = self.max_distance > 0.0 && self.walked_distance.abs() >= self.max_distance;

        if finished {
            self.stop(now);
            return None;
        }

        let animation_rate = self.animation_rate(now, last_frame);
        let offset = self.calculate_offset(animation_rate);

        let new_position = Position::new(
            sprite.position.top + offset.top,
            sprite.position.left + offset.left,
        );

        if Board::is_out_of_board(sprite, &new_position) {
            self.stop(now);
            self.interaction_active = true;
            return Some(SpriteMutation::new().hide(true));
        }

        self.walked_distance += self.position_distance(&offset);

        Some(SpriteMutation::new().position(new_position))
    }
}
