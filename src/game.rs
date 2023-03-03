use web_sys::{HtmlCanvasElement, MouseEvent};

use crate::battle_manage::BattleManager;
use crate::board::{Board, BoardLocation};
use crate::features::GameFeatures;
use crate::fps::Fps;

use crate::model::{
    BehaviorType, Callback, GameInteraction, GameMouseEvent, GameState, Position, SpriteType,
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

    last_gc: f64,
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
            last_gc: 0.0,
        }
    }

    pub fn init(&mut self, resources: Resources) {
        self.resources = resources;

        // Start game internal time
        self.game_time.start();

        // Paint home scene
        self.start_home_scene();

        self.last_gc = self.game_time.time;
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

        // Game fight
        BattleManager::manage_fight(self);

        // Draw game Sprites
        self.draw();

        // Handle Sprites interactions
        self.handle_game_interactions();

        // Internal garbage collector
        self.sprites_garbage_collector();

        SunManager::tick(self);

        self.game_time.stamp();
    }

    fn draw(&mut self) {
        self.painter.clear();

        self.sprites
            .iter_mut()
            .filter(|sprite| sprite.visible)
            .for_each(|sprite| {
                // Collect behaviors mutations
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
                self.toggle_game_behavior(true, &[BehaviorType::Click]);
            }
            GameMouseEvent::MouseUp => {
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
            Callback::StartBattleCallout => self.start_battle_callout(),
            Callback::StartBattle => self.start_battle(),
            Callback::ChooserSeedSelect => self.on_chooser_seed_click(sprite_id),
            Callback::PlantCardClick => self.on_plant_card_click(sprite_id),
            Callback::CollectSun => self.collect_sun(sprite_id),
            Callback::RemoveSun => self.remove_sprite_by_id(sprite_id),
            Callback::ReverseSun => self.reverse_sun(sprite_id),
            Callback::Plant => self.plant_on_board(sprite_id),
            Callback::AllowShovelDrag => self.allow_shovel_drag(),
            Callback::ShovelDragEnd => self.on_shovel_drag_end(),
            Callback::Shoot => self.on_plant_shoot(sprite_id),
            Callback::GenerateSunFlowerSun => self.generate_sunflower_sun(sprite_id),
            Callback::CreateZombieHead => self.show_zombie_head(sprite_id),
        }
    }

    // Scenes //
    pub fn game_over(&mut self) {
        self.painter.clear();
    }

    fn start_home_scene(&mut self) {
        self.toggle_game_behavior(false, &[BehaviorType::Collision]);

        self.reset_state();

        GameFeatures::enable_update_sun_score(false);
        GameFeatures::enable_generate_sun(false);

        HomeScene::start(self);
    }

    fn select_level(&mut self) {
        self.reset_state();

        self.state.current_level = Some(self.resources.get_level_data("1-1"));

        GameFeatures::enable_board_lines(true);

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

    pub fn start_battle_callout(&mut self) {
        BattleScene::battle_callout(self);
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

            BattleScene::deselect_seed(self, selected);
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

    pub fn allow_shovel_drag(&mut self) {
        BattleScene::allow_shovel_drag(self);
    }

    pub fn on_shovel_drag_end(&mut self) {
        self.reset_shovel();
        let dropped_location = Board::get_board_location(&self.mouse_position);
        let plant = self.get_sprite_by_location(&dropped_location);

        match plant {
            Some(plant) if plant.sprite_type == SpriteType::Plant => {
                let id = plant.id.clone();
                self.remove_sprite_by_id(&id);
            }
            _ => {}
        }
    }

    pub fn reset_shovel(&mut self) {
        let shovel_sprite = self.get_sprite_by_name_and_type("Shovel", &SpriteType::Interface);

        // Restore Shovel into it's original position.
        shovel_sprite.update_position(shovel_sprite.origin_position);
    }

    pub fn plant_on_board(&mut self, sprite_id: &String) {
        let mouse = self.mouse_position;

        // Check if current position is "active" board cell
        if !Board::is_active_board_location(&mouse) {
            self.remove_sprite_by_id(sprite_id);
            return;
        }

        let target_location = Board::get_board_location(&mouse);

        if self.is_free_board_location(sprite_id, &target_location) {
            BattleScene::create_plant(self, sprite_id);
        } else {
            self.remove_sprite_by_id(sprite_id)
        }

        self.sort_sprites();
    }

    pub fn on_plant_shoot(&mut self, sprite_id: &String) {
        let shooting_plant_location = &self.get_sprite_by_id(sprite_id).board_location.clone();

        // Check if row contains an enemy
        let has_enemy_in_row = self.sprites.iter_mut().find(|sprite| {
            sprite.visible
                && !sprite.attack_state.is_dead()
                && sprite.sprite_type == SpriteType::Zombie
                && sprite.board_location.row == shooting_plant_location.row
        });

        if has_enemy_in_row.is_some() {
            BattleScene::create_bullet(self, sprite_id)
        }
    }

    pub fn collect_sun(&mut self, sprite_id: &String) {
        self.state.sun_state.add_score(50);

        self.remove_sprite_by_id(sprite_id);
    }

    fn generate_sunflower_sun(&mut self, sprite_id: &String) {
        let sunflower_position = self.get_sprite_by_id(sprite_id).position;

        SunManager::generate_sunflower_sun(self, sunflower_position);
    }

    fn reverse_sun(&mut self, sprite_id: &String) {
        SunManager::reverse_sun(self, sprite_id);
    }

    pub fn show_zombie_head(&mut self, zombie_id: &String) {
        BattleScene::build_zombie_head(self, zombie_id)
    }

    fn sprites_garbage_collector(&mut self) {
        if self.game_time.time - self.last_gc > 1000.0 {
            self.last_gc = self.game_time.time;
            let invisible_sprites_ids = self
                .sprites
                .iter_mut()
                .filter(|sprite| !sprite.visible)
                .map(|sprite| sprite.id.clone())
                .collect::<Vec<String>>();

            self.remove_sprites_by_id(invisible_sprites_ids);
        }
    }

    // Game State Mutations //

    pub fn reset_state(&mut self) {
        self.sprites.clear();
        self.state = GameState::new();
    }

    pub fn add_sprites(&mut self, sprites: &mut Vec<Sprite>) {
        self.sprites.append(sprites);

        self.sort_sprites();
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

    pub fn remove_sprites_by_id(&mut self, sprite_ids: Vec<String>) {
        self.sprites
            .retain(|sprite| !sprite_ids.contains(&sprite.id.clone()))
    }

    pub fn remove_sprite_by_id(&mut self, sprite_id: &String) {
        self.sprites.retain(|sprite| !sprite.id.eq(sprite_id))
    }

    fn sort_sprites(&mut self) {
        self.sprites.sort_by(|a, b| a.order.cmp(&b.order));
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
            .unwrap_or_else(|| panic!("[Game Controller] Cannot find Sprite {}", &name))
    }

    pub fn get_sprite_by_id(&mut self, sprite_id: &String) -> &mut Sprite {
        self.sprites
            .iter_mut()
            .find(|sprite| sprite_id == &sprite.id)
            .unwrap_or_else(|| panic!("[Game Controller] Cannot find Sprite {}", &sprite_id))
    }

    pub fn get_sprite_by_location(&mut self, location: &BoardLocation) -> Option<&Sprite> {
        let found = self
            .sprites
            .iter()
            .filter(|sprite| sprite.sprite_type == SpriteType::Plant)
            .find(|sprite| {
                location.row == sprite.board_location.row && location.col == sprite.board_location.col
            });

        found
    }

    pub fn is_free_board_location(&mut self, sprite_id: &String, location: &BoardLocation) -> bool {
        let found = self.get_sprite_by_location(location);

        match found {
            None => true,
            Some(sprite) if &sprite.id == sprite_id => true,
            Some(_) => false,
        }
    }

    pub fn canvas(&self) -> &HtmlCanvasElement {
        &self.painter.canvas
    }
}
