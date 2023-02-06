use crate::game::Game;
use crate::log;
use crate::model::{BehaviorType, Callback, Position, SelectedSeed, SpriteType};
use crate::resource_loader::ResourceKind;
use crate::scene::PlantsChooser;
use crate::sprite::{BehaviorManager, Sprite};

pub struct BattleScene;

impl BattleScene {
    pub fn prepare(game: &mut Game) {
        let mut sprites = Sprite::create_sprites(
            vec!["BattleBackground", "BackButton"],
            &ResourceKind::Interface,
            &game.resources,
        );

        // TODO - Show Enemies (Zombies)

        // Trigger background scroll
        BehaviorManager::toggle_behaviors(
            &sprites,
            &[BehaviorType::Scroll],
            true,
            game.game_time.time,
        );

        game.add_sprites(sprites.as_mut());
    }

    pub fn enter(game: &mut Game) {
        let now = game.game_time.time;

        PlantsChooser::clear(game);

        // Trigger background reverse scroll behavior
        let background = game.get_sprite_by_name_and_type("BattleBackground", &SpriteType::Interface);
        let scroll = BehaviorManager::get_sprite_behavior(background, BehaviorType::Scroll);

        scroll.reverse(now, Callback::StartBattle);
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
        // TODO - Swap cards Callback to Plant action.
        log!("Starting Battle Scene!");
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
