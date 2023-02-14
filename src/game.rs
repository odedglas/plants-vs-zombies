use web_sys::{HtmlCanvasElement, MouseEvent};

use crate::board::Board;
use crate::features::GameFeatures;
use crate::fps::Fps;
use crate::log;
use crate::model::{
    BehaviorData, BehaviorType, Callback, GameInteraction, GameMouseEvent, GameState, Position,
    SpriteType,
};
use crate::painter::Painter;
use crate::resource_loader::Resources;
use crate::scene::{BattleScene, HomeScene, PlantsChooser};
use crate::sprite::{BehaviorManager, Sprite};
use crate::sun_manager::SunManager;
use crate::timers::GameTime;

pub struct Game {
    pub resources: Resources,
    pub painter: Painter,
    pub game_time: GameTime,
    pub mouse_position: Position,
    pub sprites: Vec<Sprite>,
    pub state: GameState,
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
        }
    }

    pub fn init(&mut self, resources: Resources) {
        self.resources = resources;

        // Start game internal time
        self.game_time.start();

        // Paint home scene
        self.start_home_scene();
    }

    pub fn init_debug_mode(&mut self, resource: Resources) {
        self.init(resource);

        self.select_level();

        GameFeatures::enable_board_lines(true);
    }

    pub fn run(&mut self) {
        let current_time = self.game_time.current_time();
        let last_frame = self.game_time.last_timestamp;

        self.fps.calc(current_time, last_frame);

        // Draw game Sprites
        self.draw();

        // Handle Sprites interactions
        self.handle_game_interactions();

        // TODO - Handle Game internal sprites garbage collection Game::gc()

        SunManager::tick(self);

        self.game_time.stamp();
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

        Board::draw(self);
        SunManager::update_sun_score(self);
    }

    // Canvas Mouse Events //

    pub fn handle_mouse_event(&mut self, event_name: GameMouseEvent, event: MouseEvent) {
        let current_mouse = Position::from_event(event);
        self.mouse_position = current_mouse;

        match event_name {
            GameMouseEvent::MouseMove => {
                self.toggle_game_behavior(true, &[BehaviorType::Hover]);
            }
            GameMouseEvent::MouseDown => {
                self.toggle_game_behavior(true, &[BehaviorType::Click, BehaviorType::Drag]);
            }
            GameMouseEvent::MouseUp => {
                self.clear_dragged_sprites();
                self.toggle_game_behavior(false, &[BehaviorType::Click, BehaviorType::Drag]);
            }
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
                GameInteraction::SpriteClick(callback, sprite_id) => {
                    self.interaction_callback(callback, sprite_id)
                }
                GameInteraction::AnimationCallback(callback, sprite_id) => {
                    self.interaction_callback(callback, sprite_id)
                }
            });
    }

    pub fn interaction_callback(&mut self, callback: &Callback, sprite_id: &String) {
        match callback {
            Callback::ShowZombieHand => self.show_zombie_hand_animation(),
            Callback::SelectLevel => self.select_level(),
            Callback::BackHome => self.start_home_scene(),
            Callback::ShowPlantsChooser => self.show_plants_chooser(),
            Callback::ResetPlantsChoose => self.reset_plants_choose(),
            Callback::EnterBattleAnimation => self.enter_battle_animation(),
            Callback::StartBattle => self.start_battle(),
            Callback::ChooserSeedSelect => self.on_chooser_seed_click(sprite_id),
            Callback::PlantCardClick => self.on_plant_card_click(sprite_id),
            Callback::CollectSun => self.collect_sun(sprite_id),
            Callback::RemoveSun => self.remove_sun(sprite_id),
        }
    }

    // Scenes //
    pub fn game_over(&mut self) {
        self.painter.clear();
    }

    fn start_home_scene(&mut self) {
        self.reset_state();

        GameFeatures::enable_update_sun_score(false);
        GameFeatures::enable_generate_sun(false);

        HomeScene::start(self);
    }

    fn select_level(&mut self) {
        self.reset_state();

        self.state.current_level = Some(self.resources.get_level_data("1-1"));

        BattleScene::prepare(self);
    }

    pub fn show_zombie_hand_animation(&mut self) {
        HomeScene::show_zombie_hand(self);
    }

    pub fn show_plants_chooser(&mut self) {
        GameFeatures::enable_update_sun_score(true);

        PlantsChooser::show(self);
    }

    pub fn reset_plants_choose(&mut self) {
        PlantsChooser::reset_selection(self);

        self.state.selected_seeds = vec![];
    }

    pub fn enter_battle_animation(&mut self) {
        BattleScene::enter(self)
    }

    pub fn start_battle(&mut self) {
        GameFeatures::enable_generate_sun(true);

        BattleScene::start(self);
    }

    pub fn on_chooser_seed_click(&mut self, clicked_sprite_id: &String) {
        let selected_seeds = self.state.selected_seeds.to_vec();

        // Each selected seed is represented as a Seed/Card tuple.
        let selected = selected_seeds.iter().find(|selected_seed| {
            &selected_seed.0 == clicked_sprite_id || &selected_seed.1 == clicked_sprite_id
        });

        if let Some(selected) = selected {
            let is_seed_click = clicked_sprite_id == &selected.0;

            // Seeds are disabled once clicked, and can be de-selected only on Card click.
            if is_seed_click {
                return;
            }

            // Deselecting
            self.state
                .selected_seeds
                .retain(|(_seed_id, card_id)| card_id != clicked_sprite_id);

            BattleScene::deselect_seed(self, &selected);
        } else {
            let card_id = BattleScene::select_seed(self, clicked_sprite_id);

            self.state
                .selected_seeds
                .push((clicked_sprite_id.clone(), card_id));
        }
    }

    pub fn on_plant_card_click(&mut self, sprite_id: &String) {
        BattleScene::create_draggable_plant(self, sprite_id);
    }

    pub fn collect_sun(&mut self, sprite_id: &String) {
        self.state.sun_state.add_score(50);

        self.remove_sun(sprite_id);
    }

    pub fn remove_sun(&mut self, sprite_id: &String) {
        self.remove_sprites_by_id(vec![sprite_id]);
    }

    // Game State Mutations //

    pub fn reset_state(&mut self) {
        self.sprites.clear();
        self.state = GameState::new();
    }

    pub fn clear_dragged_sprites(&mut self) {
        self.sprites
            .iter_mut()
            .for_each(|sprite| sprite.draggable = false)
    }

    pub fn add_sprites(&mut self, sprites: &mut Vec<Sprite>) {
        self.sprites.append(sprites);

        self.sprites.sort_by(|a, b| a.order.cmp(&b.order));
    }

    pub fn add_sprite(&mut self, sprite: Sprite) {
        let mut sprites = vec![sprite];

        self.add_sprites(sprites.as_mut());
    }

    pub fn remove_sprites_by_name(&mut self, sprites: Vec<&str>) {
        self.sprites
            .retain(|sprite| !sprites.contains(&sprite.name.trim()))
    }

    pub fn remove_sprites_by_type(&mut self, sprite_type: &SpriteType) {
        self.sprites
            .retain(|sprite| &sprite.sprite_type != sprite_type)
    }

    pub fn remove_sprites_by_id(&mut self, sprites: Vec<&str>) {
        self.sprites
            .retain(|sprite| !sprites.contains(&sprite.id.trim()))
    }

    // Getters //
    pub fn get_sprites_by_type(&mut self, sprite_type: &SpriteType) -> Vec<&mut Sprite> {
        self.sprites
            .iter_mut()
            .filter(|sprite| &sprite.sprite_type == sprite_type)
            .collect()
    }

    pub fn get_sprite_by_name_and_type(
        &mut self,
        name: &str,
        sprite_type: &SpriteType,
    ) -> &mut Sprite {
        self.sprites
            .iter_mut()
            .find(|sprite| name == sprite.name && &sprite.sprite_type == sprite_type)
            .expect(&format!("[Game Controller] Cannot find Sprite {}", &name))
    }

    pub fn get_sprite_by_id(&mut self, sprite_id: &String) -> &mut Sprite {
        self.sprites
            .iter_mut()
            .find(|sprite| sprite_id == &sprite.id)
            .expect(&format!(
                "[Game Controller] Cannot find Sprite {}",
                &sprite_id
            ))
    }

    pub fn canvas(&self) -> &HtmlCanvasElement {
        &self.painter.canvas
    }
}
