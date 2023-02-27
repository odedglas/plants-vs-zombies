use crate::model::Position;

#[derive(Debug, Clone)]
pub struct SpriteMutation {
    pub position: Option<Position>,
    pub offset: Option<Position>,
    pub hovered: Option<bool>,
    pub swap: Option<i32>,
    pub cycle_cells: Option<bool>,
    pub visible: Option<bool>,
    pub mute: Option<bool>,
    pub damage: Option<f64>,
    pub alpha: Option<f64>,
    pub walking: Option<bool>,
    pub stop_animate: Option<bool>,
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
            swap: None,
            mute: None,
            alpha: None,
            walking: None,
            stop_animate: None,
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

    pub fn hide(mut self, hide: bool) -> Self {
        self.visible = Some(!hide);

        self
    }

    pub fn damage(mut self, damage: f64) -> Self {
        self.damage = Some(damage);

        self
    }

    pub fn swap(mut self, swap_index: i32) -> Self {
        self.swap = Some(swap_index);

        self
    }

    pub fn mute(mut self, muted: bool) -> Self {
        self.mute = Some(muted);

        self
    }

    pub fn alpha(mut self, alpha: f64) -> Self {
        self.alpha = Some(alpha);

        self
    }

    pub fn stop_animate(mut self) -> Self {
        self.stop_animate = Some(true);

        self
    }
}
