use crate::game::Game;
use crate::log;
use crate::model::{BehaviorType, Callback};
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
        let background = game.get_sprite("BattleBackground");
        let scroll = BehaviorManager::get_sprite_behavior(background, BehaviorType::Scroll);

        scroll.reverse(now, Callback::StartBattle);
    }

    pub fn start(game: &mut Game) {
        log!("Starting Battle Scene!");
        todo!()
    }
}
