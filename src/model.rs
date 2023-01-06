use serde_derive::Deserialize;

/// The events being listened by our game.
#[derive(Debug, Clone, Copy)]
pub enum GameEvent {
    Mousedown,
    Mousemove,
    Mouseup,
    Mouseleave,
    Mouseenter,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct SpriteCell {
    pub left: f64,
    pub top: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LevelData {
    pub name: String,
    pub scenes: Vec<String>,
    pub flag_num: usize,
    pub plant_cards: Vec<String>,
    pub zombie_cards: Vec<String>,
}
