use js_sys::Math;

use crate::board::Board;
use crate::log;
use crate::model::{LocationType, Position, Size, SpriteCell};
use crate::sprite::Sprite;

pub struct LocationBuilder;

impl LocationBuilder {
    pub fn text_overlay_location(
        sprite: &Sprite,
        item_dimensions: Size,
        location_type: &LocationType,
    ) -> Position {
        match location_type {
            LocationType::Center => Self::place_at_center(sprite, item_dimensions),
            LocationType::Top => Self::place_at_top(sprite, item_dimensions),
        }
    }

    pub fn sun_location() -> Position {
        Position::new(
            Self::rand_within_rand(0.0, 80.0),
            Self::rand_within_rand(100.0, 750.0),
        )
    }

    pub fn plant_location(plant_cell: &SpriteCell, mouse: &Position) -> Position {
        let location = Board::get_board_location(mouse);

        Board::get_board_placement(plant_cell, location.row, location.col)
    }

    pub fn zombie_location(zombie_cell: &SpriteCell, row: usize) -> Position {
        let start_col = 10;
        let start_row = ((row) % 5) + 1;

        let board_position = Board::get_board_placement(zombie_cell, start_row, start_col);

        let x_offset = Self::random_offset(0, 30);

        Position::new(board_position.top, board_position.left + x_offset)
    }

    pub fn is_active_board_location(position: &Position) -> bool {
        let location = Board::get_board_location(position);

        location.col >= 2
    }

    fn rand_within_rand(min: f64, max: f64) -> f64 {
        let min = Math::ceil(min);
        let max = Math::floor(max);

        Math::floor(Math::random() * (max - min + 1.0)) + min
    }

    fn random_offset(min: usize, max: usize) -> f64 {
        let x_adjustment = Self::rand_within_rand(min as f64, max as f64);

        let direction = match Self::rand_within_rand(0.0, 1.0) > 0.5 {
            true => 1.0,
            false => -1.0,
        };

        x_adjustment * direction
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
