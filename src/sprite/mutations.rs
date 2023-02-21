use crate::model::Position;

pub struct SpriteMutation {
    pub position: Option<Position>,
    pub offset: Option<Position>,
    pub hovered: Option<bool>,
    pub cycle_cells: Option<bool>,
    pub visible: Option<bool>,
    pub damage: Option<f64>,
}

impl SpriteMutation {
    pub fn new() -> Self {
        Self {
            position: None,
            offset: None,
            hovered: None,
            cycle_cells: None,
            visible: None,
            damage: None,
        }
    }

    pub fn position(mut self, position: Position) -> Self {
        self.position = Some(position);

        self
    }

    pub fn offset(mut self, offset: Position) -> Self {
        self.offset = Some(offset);

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

    pub fn hide(mut self) -> Self {
        self.visible = Some(false);

        self
    }

    pub fn damage(mut self, damage: f64) -> Self {
        self.damage = Some(damage);

        self
    }
}
