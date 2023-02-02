use crate::game::Game;
use crate::model::BehaviorType;
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

        // Show Enemies (Zombies)

        // Trigger background scroll
        BehaviorManager::toggle_behaviors(
            sprites.iter(),
            &[BehaviorType::Scroll],
            true,
            game.game_time.time,
        );

        game.add_sprites(sprites.as_mut());
    }

    pub fn start(game: &mut Game) {
        PlantsChooser::clear(game);

        // TODO - Game find by name.
        /*        BehaviorManager::toggle_behaviors(
            sprites.iter(),
            &[BehaviorType::Scroll],
            true,
            game.game_time.time,
        );*/
    }
}
