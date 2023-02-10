use crate::game::Game;
use crate::model::{Dimensions, Position, SpriteCell};

pub struct BoardLocation {
    row: usize,
    col: usize,
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
        ROW_Y_COORD.iter().for_each(|cord| {
            game.painter
                .draw_line(&Position::new(*cord, 0.0), &Position::new(*cord, 1400.0));
        });

        COL_X_COORD.iter().for_each(|cord| {
            game.painter
                .draw_line(&Position::new(0.0, *cord), &Position::new(1000.0, *cord));
        });
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
}
