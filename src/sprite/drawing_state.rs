use crate::log;
use crate::model::{Position, SpriteCell};
use crate::sprite::Sprite;

#[derive(Debug, Default)]
pub struct DrawingState {
    pub cells: Vec<SpriteCell>,
    pub swap_cells: Vec<Vec<SpriteCell>>,
    pub swap_index: Option<usize>,
    pub active_cell: usize,
    pub scale: f64,
    pub offset: Position,
    pub alpha: f64,
}

impl DrawingState {
    pub fn new(
        cells: Vec<SpriteCell>,
        swap_cells: Vec<Vec<SpriteCell>>,
        scale: f64,
        offset: Position,
    ) -> Self {
        Self {
            scale,
            cells,
            swap_cells,
            offset,
            alpha: 1.0,
            ..DrawingState::default()
        }
    }

    pub fn get_cells(&self) -> &Vec<SpriteCell> {
        match self.swap_index {
            None => &self.cells,
            Some(index) => &self.swap_cells[index],
        }
    }

    pub fn get_active_cell(sprite: &Sprite) -> &SpriteCell {
        let drawing_state = &sprite.drawing_state;

        let cells = drawing_state.get_cells();

        let cell = cells.get(drawing_state.active_cell).expect(&format!(
            "[Sprite] Cannot get drawing state cell of {} / {} / {:?}",
            sprite.name, drawing_state.active_cell, drawing_state.swap_index
        ));

        return cell;
    }

    pub fn swap(&mut self, swap_index: usize) {
        if self.swap_index != Some(swap_index) {
            self.active_cell = 0;
            self.swap_index = Some(swap_index);
        }
    }

    pub fn reset_swap(&mut self) {
        if self.swap_index.is_some() {
            self.active_cell = 0;
            self.swap_index = None;
        }
    }

    pub fn hover(&mut self, hover: bool) {
        let active_index = match hover {
            true => 1,
            false => 0,
        };

        self.set_cell(active_index);
    }

    pub fn cycle_cells(&mut self) {
        let current = self.active_cell;
        let max = self.get_cells().len();

        let next_index = match current < max - 1 {
            true => current + 1,
            false => 0,
        };

        self.set_cell(next_index);
    }

    pub fn in_last_cell(&self) -> bool {
        self.active_cell == self.get_cells().len() - 1
    }

    fn set_cell(&mut self, index: usize) {
        self.active_cell = index;
    }
}
