use crate::constants::{CANVAS_HEIGHT_F64, CANVAS_WIDTH_F64};
use crate::features::GameFeatures;
use crate::game::Game;
use crate::model::{Dimensions, Position, SpriteCell};
use crate::sprite::{DrawingState, Sprite};

#[derive(Debug, Clone, Copy)]
pub struct BoardLocation {
    pub row: usize,
    pub col: usize,
}

impl BoardLocation {
    pub fn new(row: usize, col: usize) -> Self {
        BoardLocation { row, col }
    }
}

pub struct Board;

pub const ROW_Y_COORD: [f64; 6] = [75.0, 175.0, 275.0, 380.0, 475.0, 575.0];

pub const COL_X_COORD: [f64; 17] = [
    100.0, 140.0, 220.0, 295.0, 379.0, 460.0, 540.0, 625.0, 695.0, 775.0, 855.0, 935.0, 1015.0,
    1095.0, 1175.0, 1255.0, 1335.0,
];

impl Board {
    pub fn draw(game: &mut Game) {
        if GameFeatures::show_board_lines() {
            ROW_Y_COORD.iter().for_each(|cord| {
                game.painter
                    .draw_line(&Position::new(*cord, 0.0), &Position::new(*cord, 1400.0));
            });

            COL_X_COORD.iter().for_each(|cord| {
                game.painter
                    .draw_line(&Position::new(0.0, *cord), &Position::new(1000.0, *cord));
            });
        }
    }

    pub fn get_cell_dimensions(row: usize, col: usize) -> Dimensions {
        let right = COL_X_COORD[col];
        let left = match col > 0 {
            true => COL_X_COORD[col - 1],
            false => 0.0,
        };

        let bottom = ROW_Y_COORD[row];
        let top = match row > 0 {
            true => ROW_Y_COORD[row - 1],
            false => 0.0,
        };

        SpriteCell {
            top,
            left,
            width: right - left,
            height: bottom - top,
        }
    }

    pub fn get_board_placement(cell: &SpriteCell, row: usize, col: usize) -> Position {
        let dimensions = Self::get_cell_dimensions(row, col);

        let center_x = dimensions.left + (dimensions.width - cell.width) / 2.0;
        let bottom = dimensions.top - (cell.height - dimensions.height) - 3.5;

        Position::new(bottom, center_x)
    }

    pub fn get_board_location(position: &Position) -> BoardLocation {
        let row = ROW_Y_COORD
            .into_iter()
            .position(|row_cord| position.top <= row_cord)
            .unwrap_or(0);

        let col = COL_X_COORD
            .into_iter()
            .position(|col_cord| position.left <= col_cord)
            .unwrap_or(0);

        BoardLocation::new(row, col)
    }

    pub fn is_active_board_location(position: &Position) -> bool {
        let location = Self::get_board_location(position);

        location.col > 1 && location.col <= 9 && location.row > 0 && location.row <= 5
    }

    pub fn is_out_of_board(sprite: &Sprite, position: &Position) -> bool {
        let cell = DrawingState::get_active_cell(sprite);

        position.top + cell.height < 0.0
            || position.left + cell.width < 0.0
            || position.left > CANVAS_WIDTH_F64
            || position.top > CANVAS_HEIGHT_F64
    }
}
