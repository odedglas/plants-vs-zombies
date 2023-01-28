use std::fmt;

use serde_derive::Deserialize;
use web_sys::MouseEvent;

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
pub enum GameMouseEvent {
    MouseDown,
    MouseMove,
    MouseUp,
    MouseLeave,
}

impl fmt::Display for GameMouseEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub enum Callback {
    StartBattleScene,
}

#[derive(Debug)]
pub enum GameInteraction {
    SpriteClick(Callback),
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
    pub exact_outlines: bool,
    pub behaviors: Vec<BehaviorData>,
}

impl Default for SpriteData {
    fn default() -> Self {
        Self {
            position: vec![Position::default()],
            order: 1,
            scale: 1.0,
            exact_outlines: false,
            behaviors: vec![],
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq)]
pub enum BehaviorType {
    Hover,
    Click,
}

impl Default for BehaviorType {
    fn default() -> BehaviorType {
        BehaviorType::Click
    }
}

impl BehaviorType {
    pub fn from_string(name: &str) -> BehaviorType {
        match name {
            "Click" => BehaviorType::Click,
            "Hover" => BehaviorType::Hover,
            _ => BehaviorType::default(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct BehaviorData {
    pub name: String,
    pub callback: Option<Callback>
}

#[derive(Debug, Default, PartialEq, Clone, Copy, Deserialize)]
pub struct Position {
    pub left: f64,
    pub top: f64,
}

impl Position {
    pub fn new(top: f64, left: f64) -> Self {
        Self { top, left }
    }

    pub fn from_event(event: MouseEvent) -> Self {
        Self {
            top: event.offset_y() as f64,
            left: event.offset_x() as f64,
        }
    }
}

#[derive(Debug, Default)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

impl From<&SpriteCell> for Size {
    fn from(cell: &SpriteCell) -> Size {
        Size {
            width: cell.width,
            height: cell.height,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct LevelData {
    pub name: String,
    pub scenes: Vec<String>,
    pub flag_num: usize,
    pub plants_options: Vec<String>,
    pub zombies: Vec<String>,
}
