use crate::model::Position;

pub struct SpriteMutation {
    pub position: Option<Position>,
    pub hovered: Option<bool>,
    pub cycle_cells: Option<bool>,
}

impl SpriteMutation {
    pub fn new() -> Self {
        Self {
            position: None,
            hovered: None,
            cycle_cells: None,
        }
    }

    pub fn position(mut self, position: Position) -> Self {
        self.position = Some(position);

        self
    }

    pub fn hovered(mut self, hovered: bool) -> Self {
        self.hovered = Some(hovered);

        self
    }

    pub fn cycle(mut self) -> Self {
        self.cycle_cells = Some(true);

        self
    }
}
