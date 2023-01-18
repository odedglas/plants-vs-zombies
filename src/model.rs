use std::fmt;

use serde_derive::Deserialize;

#[derive(Debug, Default)]
pub struct GameState {
    pub sun: usize,
    pub current_level: Option<LevelData>,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            sun: 600,
            current_level: None,
        }
    }
}

/// The events being listened by our game.
#[derive(Debug, Clone, Copy)]
pub enum GameEvent {
    Mousedown,
    Mousemove,
    Mouseup,
    Mouseleave,
    Mouseenter,
}

impl fmt::Display for GameEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

/// Sprite cell represents a Sprite given possible states position pointing to a respective interface asset.
#[derive(Debug, Default, Clone, Deserialize)]
pub struct SpriteCell {
    pub left: f64,
    pub top: f64,
    pub width: f64,
    pub height: f64,
}

/// Sprite data represents the meta data of a given Sprite
#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct SpriteData {
    pub position: Vec<Position>,
    pub order: usize,
    pub scale: f64,
}

impl Default for SpriteData {
    fn default() -> Self {
        Self {
            position: vec![Position::default()],
            order: 1,
            scale: 1.0,
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone, Copy, Deserialize)]
pub struct Position {
    pub left: f64,
    pub top: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LevelData {
    pub name: String,
    pub scenes: Vec<String>,
    pub flag_num: usize,
    pub plants_options: Vec<String>,
    pub zombies: Vec<String>,
}
