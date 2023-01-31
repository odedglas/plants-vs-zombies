use crate::model::{Position, Size};
use crate::sprite::Sprite;

pub struct LocationBuilder;

impl LocationBuilder {
    pub fn place_at_center(sprite: &Sprite, item_dimensions: Size) -> Position {
        let target_dimensions = sprite.dimensions();

        let center_x =
            target_dimensions.left + (target_dimensions.width - item_dimensions.width) / 2.0;
        let center_y =
            target_dimensions.top + (target_dimensions.height - item_dimensions.height) / 2.0;

        Position::new(center_y, center_x)
    }
}
