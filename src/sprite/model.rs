use crate::model::Position;

pub struct SpriteMutation {
    pub position: Option<Position>,
    pub hovered: Option<bool>,
    pub clicked: Option<bool>,
}

impl SpriteMutation {
    pub fn new(position: Option<Position>, hovered: Option<bool>, clicked: Option<bool>) -> Self {
        Self {
            position,
            hovered,
            clicked,
        }
    }
}
