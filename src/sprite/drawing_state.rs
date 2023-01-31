use crate::model::SpriteCell;
use crate::sprite::Sprite;

#[derive(Debug, Default)]
pub struct DrawingState {
    pub cells: Vec<SpriteCell>,
    pub active_cell: usize,
    pub scale: f64,
}

impl DrawingState {
    pub fn new(cells: Vec<SpriteCell>, scale: f64) -> Self {
        Self {
            scale,
            cells,
            ..DrawingState::default()
        }
    }

    pub fn get_active_cell(sprite: &Sprite) -> &SpriteCell {
        let drawing_state = &sprite.drawing_state;
        let cell = drawing_state
            .cells
            .get(drawing_state.active_cell)
            .expect(&format!(
                "[Sprite] Cannot get drawing state cell of {} / {}",
                sprite.name, drawing_state.active_cell
            ));

        return cell;
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
        let max = self.cells.len();

        let next_index = match current < max - 1 {
            true => current + 1,
            false => 0,
        };

        self.set_cell(next_index);
    }

    pub fn in_last_cell(&self) -> bool {
        self.active_cell == self.cells.len() - 1
    }

    fn set_cell(&mut self, index: usize) {
        self.active_cell = index;
    }
}
