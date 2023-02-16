use crate::game::Game;
use crate::location_builder::LocationBuilder;
use crate::log;
use crate::model::Callback::PlantCardClick;
use crate::model::{BehaviorData, BehaviorType, Callback, Position, SelectedSeed, SpriteType};
use crate::resource_loader::ResourceKind;
use crate::scene::PlantsChooser;
use crate::sprite::{BehaviorManager, Click, DrawingState, Scroll, Sprite};

pub struct BattleScene;

impl BattleScene {
    fn build_background(game: &mut Game) {
        let mut sprites = Sprite::create_sprites(
            vec![
                "BattleBackground",
                "BackButton",
                "FlagMeterEmpty",
                "FlagMeterParts1",
                "FlagMeterLevelProgress",
            ],
            &ResourceKind::Interface,
            &game.resources,
        );

        // Trigger background scroll
        BehaviorManager::toggle_behaviors(
            &sprites,
            &[BehaviorType::Scroll],
            true,
            game.game_time.time,
        );

        game.add_sprites(sprites.as_mut());
    }

    fn build_zombies(game: &mut Game) {
        let mut zombies = Sprite::create_sprites(
            game.state
                .get_level()
                .zombies
                .iter()
                .map(|zombie_name| zombie_name.trim())
                .collect(),
            &ResourceKind::Zombie,
            &game.resources,
        );

        BehaviorManager::toggle_behaviors(
            &zombies,
            &[BehaviorType::Animate],
            true,
            game.game_time.time,
        );

        // Set Zombie random start position
        zombies.iter_mut().enumerate().for_each(|(index, zombie)| {
            let zombie_cell = DrawingState::get_active_cell(zombie);

            zombie.update_position(LocationBuilder::zombie_location(zombie_cell, index))
        });

        game.add_sprites(zombies.as_mut());
    }

    pub fn prepare(game: &mut Game) {
        Self::build_background(game);

        Self::build_zombies(game);
    }

    pub fn enter(game: &mut Game) {
        let now = game.game_time.time;

        PlantsChooser::clear(game);

        // Trigger background reverse scroll behavior
        let background = game.get_sprite_by_name_and_type("BattleBackground", &SpriteType::Interface);
        let scroll = BehaviorManager::get_sprite_behavior(background, BehaviorType::Scroll);

        scroll
            .as_any()
            .downcast_mut::<Scroll>()
            .unwrap()
            .reverse(now, Callback::StartBattle);
    }

    pub fn select_seed(game: &mut Game, seed_id: &String) -> String {
        let seed = game.get_sprite_by_id(seed_id);
        let seed_name = seed.name.clone();

        seed.drawing_state.hover(true);

        Self::add_plant_card(game, &seed_name)
    }

    pub fn deselect_seed(game: &mut Game, selected_seed: &SelectedSeed) {
        let seed_sprite = game.get_sprite_by_id(&selected_seed.0);
        seed_sprite.drawing_state.hover(false);

        game.remove_sprites_by_id(vec![&selected_seed.1]);

        Self::update_selected_cards_layout(game);
    }

    pub fn start(game: &mut Game) {
        let mut scene_sprites = Sprite::create_sprites(
            vec!["SunScore", "Shovel", "ShovelBack"],
            &ResourceKind::Interface,
            &game.resources,
        );

        Self::swap_plant_cards_action(game);

        game.add_sprites(scene_sprites.as_mut());
    }

    pub fn create_draggable_plant(game: &mut Game, sprite_id: &String) {
        let mouse = game.mouse_position.clone();
        let card_sprite = game.get_sprite_by_id(sprite_id);

        let original_position = card_sprite.position.clone();
        let plant_name = card_sprite.name.clone();

        let mut plant =
            Sprite::create_sprite(&plant_name, &ResourceKind::Plant, &game.resources).remove(0);
        let plant_cell = DrawingState::get_active_cell(&plant);

        // Original card can be wider than the actual Plant
        let drag_adjustment = match mouse.left > (plant.position.left + plant_cell.width) {
            true => mouse.left - (plant.position.left + plant_cell.width),
            false => 0.0,
        };

        let mut drag_behavior = BehaviorManager::create(
            &BehaviorData::new("Drag".to_string(), Callback::Plant),
            String::from(&plant.id),
        );

        drag_behavior.toggle(true, game.game_time.time);
        plant.behaviors.borrow_mut().push(drag_behavior);

        plant.update_position(Position::new(
            original_position.top,
            original_position.left + drag_adjustment,
        ));
        plant.order = 10; // TODO, Drag order based on behavior?

        game.add_sprite(plant);
    }

    pub fn allow_shovel_drag(game: &mut Game) {
        let now = game.game_time.time;
        let shovel_sprite = game.get_sprite_by_name_and_type("Shovel", &SpriteType::Interface);

        let drag = BehaviorManager::get_sprite_behavior(shovel_sprite, BehaviorType::Drag);

        drag.start(now);
    }

    fn swap_plant_cards_action(game: &mut Game) {
        let mut plant_cards = game.get_sprites_by_type(&SpriteType::Card);
        plant_cards.iter_mut().for_each(|card| {
            let click = BehaviorManager::get_sprite_behavior(card, BehaviorType::Click);

            click.as_any().downcast_mut::<Click>().unwrap().callback = PlantCardClick;
        });
    }

    fn add_plant_card(game: &mut Game, seed_name: &String) -> String {
        let current_cards = game.state.selected_seeds.len();

        let mut plant =
            Sprite::create_sprite(seed_name, &ResourceKind::Card, &game.resources).remove(0);

        let plant_id = plant.id.clone();

        plant.drawing_state.scale = 1.0;
        plant.update_position(Position::new(60.0 * current_cards as f64, 0.0));

        game.add_sprite(plant);

        plant_id
    }

    fn update_selected_cards_layout(game: &mut Game) {
        let mut count = 0;
        let selected_seeds = &game.state.selected_seeds.to_vec();

        selected_seeds.iter().for_each(|(_seed_id, card_id)| {
            let card_sprite = game.get_sprite_by_id(card_id);

            card_sprite.update_position(Position::new(60.0 * count as f64, 0.0));

            count += 1;
        });
    }
}
