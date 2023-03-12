use std::fmt;

use serde_derive::Deserialize;
use web_sys::{MouseEvent, TextMetrics};

use crate::resource_loader::ResourceKind;
use crate::sun_manager::SunState;

pub type SelectedSeed = (String, String);
pub type Dimensions = SpriteCell;

#[derive(Debug, Default)]
pub struct GameState {
    pub sun_state: SunState,
    pub current_level: Option<LevelData>,
    pub selected_seeds: Vec<SelectedSeed>,
    pub lost_lawn_cleaners: usize,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            sun_state: SunState::new(),
            current_level: None,
            selected_seeds: vec![],
            lost_lawn_cleaners: 0,
        }
    }

    pub fn get_level(&self) -> LevelData {
        match &self.current_level {
            Some(level) => level.clone(),
            None => LevelData::new(),
        }
    }
}

/// The HTML Canvas events being listened by our game.
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
    StartBattleCallout,
    StartBattle,
    ChooserSeedSelect,
    PlantCardClick,
    RemoveSun,
    CollectSun,
    ReverseSun,
    Plant,
    AllowShovelDrag,
    ShovelDragEnd,
    Shoot,
    GenerateSunFlowerSun,
    OnZombieDeath,
    LawnCleanerLost,
}

impl Default for Callback {
    fn default() -> Callback {
        Callback::ShowZombieHand
    }
}

type SpriteId = String;

#[derive(Debug)]
pub enum GameInteraction {
    SpriteClick(Callback, SpriteId),
    AnimationCallback(Callback, SpriteId),
}

#[derive(Debug)]
pub enum Plant {
    PeaShooter,
    SnowPea,
}

impl Plant {
    pub fn from_name(name: &str) -> Plant {
        match name {
            "PeaShooter" => Plant::PeaShooter,
            "SnowPea" => Plant::SnowPea,
            _ => Plant::PeaShooter,
        }
    }

    pub fn bullet_type(plant: &Plant) -> &str {
        match plant {
            Plant::PeaShooter => "NormalBullet",
            Plant::SnowPea => "SnowBullet",
        }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum SpriteType {
    Zombie,
    Plant,
    Interface,
    Card,
    Seed,
    Bullet,
    LawnCleaner,
    Meta,
}

impl SpriteType {
    pub fn from_kind(kind: &ResourceKind) -> Self {
        match kind {
            ResourceKind::Card => SpriteType::Card,
            ResourceKind::Interface => SpriteType::Interface,
            ResourceKind::Plant => SpriteType::Plant,
            ResourceKind::Zombie => SpriteType::Zombie,
            ResourceKind::Level => SpriteType::Meta,
        }
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

#[derive(Debug, Copy, Clone, Deserialize, PartialEq)]
pub enum AttackEffect {
    TurnIntoFireBullet,
}

/// Sprite data represents the meta data of a given Sprite
#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct SpriteData {
    pub position: Vec<Position>,
    pub order: usize,
    pub scale: f64,
    pub exact_outlines: bool,
    pub life: f64,
    pub damage: f64,
    pub attack_effect: Option<AttackEffect>,
    pub sun_cost: usize,
    pub draw_offset: Position,
    pub swap_cells: Vec<String>,
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
            life: 100.0,
            damage: 0.0,
            attack_effect: None,
            sun_cost: 0,
            exact_outlines: false,
            behaviors: vec![],
            swap_cells: vec![],
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
    Walk,
    Drag,
    Interval,
    Collision,
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
            "Walk" => BehaviorType::Walk,
            "Drag" => BehaviorType::Drag,
            "Interval" => BehaviorType::Interval,
            "Collision" => BehaviorType::Collision,
            _ => BehaviorType::default(),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, Deserialize)]
pub struct Velocity {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Default, Clone, Copy, Deserialize)]
pub struct CollisionMargin {
    pub left: usize,
    pub right: usize,
    pub top: usize,
    pub bottom: usize,
}

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(default)]
pub struct BehaviorData {
    pub name: String,
    pub rate: f64,
    pub distance: f64,
    pub callback: Option<Callback>,
    pub callback_delay: Option<f64>,
    pub max_cycles: Option<usize>,
    pub velocity: Option<Velocity>,
    pub interval: Option<f64>,
    pub collision_margin: Option<CollisionMargin>,
}

impl BehaviorData {
    pub fn new(name: String, callback: Callback) -> Self {
        BehaviorData {
            name,
            callback: Some(callback),
            ..BehaviorData::default()
        }
    }
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
    pub color: Option<String>,
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
