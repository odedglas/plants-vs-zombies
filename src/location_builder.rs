use js_sys::Math;

use crate::model::{LocationType, Position, Size};
use crate::sprite::Sprite;

pub struct LocationBuilder;

impl LocationBuilder {
    pub fn locate_text_overlay(
        sprite: &Sprite,
        item_dimensions: Size,
        location_type: &LocationType,
    ) -> Position {
        match location_type {
            LocationType::Center => Self::place_at_center(sprite, item_dimensions),
            LocationType::Top => Self::place_at_top(sprite, item_dimensions),
        }
    }

    pub fn locate_sun() -> Position {
        Position::new(
            Self::rand_within_rand(0.0, 80.0),
            Self::rand_within_rand(100.0, 750.0),
        )
    }

    pub fn rand_within_rand(min: f64, max: f64) -> f64 {
        let min = Math::ceil(min);
        let max = Math::floor(max);

        Math::floor(Math::random() * (max - min + 1.0)) + min
    }

    pub fn create_row_layout(
        initial_position: &Position,
        amount: usize,
        max: usize,
        item_size: Size,
    ) -> Vec<Position> {
        let mut row_layout = vec![];

        for i in 0..amount {
            let (row, col) = (i / max, i % max);

            row_layout.push(Position::new(
                initial_position.top + item_size.height * row as f64,
                initial_position.left + item_size.width * col as f64,
            ));
        }

        row_layout
    }

    fn place_at_center(sprite: &Sprite, item_dimensions: Size) -> Position {
        let target_dimensions = sprite.dimensions();

        let center_x =
            target_dimensions.left + (target_dimensions.width - item_dimensions.width) / 2.0;
        let center_y =
            target_dimensions.top + (target_dimensions.height - item_dimensions.height) / 2.0;

        Position::new(center_y, center_x)
    }

    fn place_at_top(sprite: &Sprite, item_dimensions: Size) -> Position {
        let target_dimensions = sprite.dimensions();

        let center_x =
            target_dimensions.left + (target_dimensions.width - item_dimensions.width) / 2.0;

        Position::new(target_dimensions.top, center_x)
    }
}
