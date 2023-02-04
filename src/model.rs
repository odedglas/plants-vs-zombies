use std::fmt;

use serde_derive::Deserialize;
use web_sys::{MouseEvent, TextMetrics};

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
    ShowZombieHand,
    SelectLevel,
    BackHome,
    ShowPlantsChooser,
    ResetPlantsChoose,
    EnterBattleAnimation,
    StartBattle,
}

impl Default for Callback {
    fn default() -> Callback {
        Callback::ShowZombieHand
    }
}

#[derive(Debug)]
pub enum GameInteraction {
    SpriteClick(Callback),
    AnimationCallback(Callback),
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
    pub draw_offset: Position,
    pub behaviors: Vec<BehaviorData>,
    pub text_overlay: Option<TextOverlayData>,
}

impl Default for SpriteData {
    fn default() -> Self {
        Self {
            position: vec![Position::default()],
            draw_offset: Position::default(),
            order: 1,
            scale: 1.0,
            exact_outlines: false,
            behaviors: vec![],
            text_overlay: None,
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq)]
pub enum BehaviorType {
    Hover,
    Click,
    Animate,
    Scroll,
}

impl Default for BehaviorType {
    fn default() -> BehaviorType {
        BehaviorType::Hover
    }
}

impl BehaviorType {
    pub fn from_string(name: &str) -> BehaviorType {
        match name {
            "Click" => BehaviorType::Click,
            "Hover" => BehaviorType::Hover,
            "Animate" => BehaviorType::Animate,
            "Scroll" => BehaviorType::Scroll,
            _ => BehaviorType::default(),
        }
    }
}

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(default)]
pub struct BehaviorData {
    pub name: String,
    pub duration: f64,
    pub distance: f64,
    pub callback: Option<Callback>,
    pub callback_delay: Option<f64>,
    pub max_cycles: Option<usize>,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub enum LocationType {
    Center,
    Top,
}

impl Default for LocationType {
    fn default() -> Self {
        LocationType::Center
    }
}

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(default)]
pub struct TextOverlayData {
    pub text: String,
    pub size: usize,
    pub offset: Option<Position>,
    pub location_type: LocationType,
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

impl Size {
    pub fn new(width: f64, height: f64) -> Self {
        Size { width, height }
    }
}

impl From<&SpriteCell> for Size {
    fn from(cell: &SpriteCell) -> Size {
        Size {
            width: cell.width,
            height: cell.height,
        }
    }
}

impl From<TextMetrics> for Size {
    fn from(text_metrics: TextMetrics) -> Self {
        Size::new(
            text_metrics.width(),
            text_metrics.font_bounding_box_descent(),
        )
    }
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct LevelData {
    pub name: String,
    pub flag_num: usize,
    pub plant_cards: Vec<String>,
    pub zombies: Vec<String>,
}

impl LevelData {
    pub fn new() -> Self {
        LevelData {
            ..LevelData::default()
        }
    }
}
