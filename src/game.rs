use std::borrow::Borrow;

use web_sys::{HtmlCanvasElement, MouseEvent};

use crate::fps::Fps;
use crate::log;
use crate::model::{
    BehaviorType, Callback, GameInteraction, GameMouseEvent, GameState, LevelData, Position,
};
use crate::painter::Painter;
use crate::resource_loader::{ResourceKind, Resources};
use crate::scene::{BattleScene, HomeScene, PlantsChooser};
use crate::sprite::{BehaviorManager, Sprite};
use crate::timers::GameTime;

pub struct Game {
    pub resources: Resources,
    pub painter: Painter,
    pub game_time: GameTime,
    pub mouse_position: Position,
    pub selected_level: LevelData,
    pub sprites: Vec<Sprite>,
    state: GameState,
    fps: Fps,
}

impl Game {
    pub fn new() -> Game {
        Game {
            painter: Painter::new(),
            resources: Resources::new(),
            game_time: GameTime::new(),
            state: GameState::new(),
            fps: Fps::new(),
            mouse_position: Position::new(0.0, 0.0),
            sprites: vec![],
            selected_level: LevelData::new(),
        }
    }

    pub fn init(&mut self, resources: Resources) {
        self.resources = resources;

        // Start game internal time
        self.game_time.start();

        // Paint home scene
        self.start_home_scene();
    }

    pub fn run(&mut self) {
        let current_time = self.game_time.current_time();
        self.fps.calc(current_time);

        // Draw game Sprites
        self.draw();

        // Handle Sprites interactions
        self.handle_game_interactions();

        // TODO - Handle Game internal sprites garbage collection Game::gc()

        // TODO - SunGenerator::tick()

        self.fps.set(current_time);
    }

    fn draw(&mut self) {
        self.painter.clear();

        self.sprites.iter_mut().for_each(|sprite| {
            // Collect mutations
            let mutations = BehaviorManager::run(
                sprite,
                &self.game_time,
                &self.mouse_position,
                &self.painter.context,
            );

            // Apply on Sprite
            sprite.apply_mutation(mutations);

            self.painter.draw_sprite(sprite);
        });
    }

    // Canvas Mouse Events //

    pub fn handle_mouse_event(&mut self, event_name: GameMouseEvent, event: MouseEvent) {
        let current_mouse = Position::from_event(event);
        self.mouse_position = current_mouse;

        match event_name {
            GameMouseEvent::MouseMove => self.toggle_game_behavior(true, &[BehaviorType::Hover]),
            GameMouseEvent::MouseDown => self.toggle_game_behavior(true, &[BehaviorType::Click]),
            GameMouseEvent::MouseUp => self.toggle_game_behavior(false, &[BehaviorType::Click]),
            GameMouseEvent::MouseLeave => self.toggle_game_behavior(false, &[BehaviorType::Hover]),
        }
    }

    pub fn toggle_game_behavior(&mut self, active: bool, types: &[BehaviorType]) {
        BehaviorManager::toggle_behaviors(&self.sprites, types, active, self.game_time.time)
    }

    // Game Actions //

    pub fn handle_game_interactions(&mut self) {
        let game_interactions = self
            .sprites
            .iter_mut()
            .flat_map(|sprite| BehaviorManager::collect_interactions(sprite))
            .collect::<Vec<GameInteraction>>();

        game_interactions
            .iter()
            .for_each(|interaction| match interaction {
                GameInteraction::SpriteClick(callback) => self.interaction_callback(callback),
                GameInteraction::AnimationCallback(callback) => self.interaction_callback(callback),
            });
    }

    pub fn interaction_callback(&mut self, callback: &Callback) {
        match callback {
            Callback::ShowZombieHand => self.show_zombie_hand_animation(),
            Callback::SelectLevel => self.select_level(),
            Callback::BackHome => self.start_home_scene(),
            Callback::ShowPlantsChooser => self.show_plants_chooser(),
            Callback::ResetPlantsChoose => self.reset_plants_choose(),
            Callback::EnterBattleAnimation => self.enter_battle_animation(),
            Callback::StartBattle => self.start_battle(),
            Callback::ToggleCardSelection => self.toggle_card_selection(),
        }
    }

    // Scenes //
    pub fn game_over(&mut self) {
        self.painter.clear();
    }

    fn start_home_scene(&mut self) {
        self.reset_state();

        HomeScene::start(self);
    }

    fn select_level(&mut self) {
        self.reset_state();

        self.selected_level = self.resources.get_level_data("1-1");

        BattleScene::prepare(self);
    }

    pub fn show_zombie_hand_animation(&mut self) {
        HomeScene::show_zombie_hand(self);
    }

    pub fn show_plants_chooser(&mut self) {
        PlantsChooser::show(self);
    }

    pub fn reset_plants_choose(&mut self) {
        log!("Game scene - Reset PlantsChooser");
    }

    pub fn enter_battle_animation(&mut self) {
        BattleScene::enter(self)
    }

    pub fn start_battle(&mut self) {
        BattleScene::start(self);
    }

    pub fn toggle_card_selection(&mut self) {
        log!("Game scene - Card selected");
    }

    // Game State Mutations //

    pub fn reset_state(&mut self) {
        self.sprites.clear();
        self.state = GameState::new();
        self.selected_level = LevelData::new();
    }

    pub fn add_sprites(&mut self, sprites: &mut Vec<Sprite>) {
        self.sprites.append(sprites);

        self.sprites.sort_by(|a, b| a.order.cmp(&b.order));
    }

    pub fn add_sprite(&mut self, sprite: Sprite) {
        let mut sprites = vec![sprite];

        self.add_sprites(sprites.as_mut());
    }

    pub fn remove_sprites(&mut self, sprites: Vec<&str>) {
        self.sprites
            .retain(|sprite| !sprites.contains(&sprite.name.trim()))
    }

    // Getters //
    pub fn get_sprite(&mut self, sprite_name: &str, kind: &ResourceKind) -> &mut Sprite {
        self.sprites
            .iter_mut()
            .find(|sprite| sprite_name == sprite.name && &sprite.kind == kind)
            .expect(&format!(
                "[Game Controller] Cannot find Sprite {}",
                &sprite_name
            ))
    }

    pub fn canvas(&self) -> &HtmlCanvasElement {
        &self.painter.canvas
    }
}
