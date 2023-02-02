use crate::game::Game;
use crate::log;
use crate::model::BehaviorType;
use crate::resource_loader::ResourceKind;
use crate::sprite::{BehaviorManager, Sprite};

pub struct BattleScene;

impl BattleScene {
    pub fn start(game: &mut Game) {
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

    pub fn show_plants_chooser(game: &mut Game) {
        let mut sprites = Sprite::create_sprites(
            vec!["SeedChooserBackground"],
            &ResourceKind::Interface,
            &game.resources,
        );

        game.add_sprites(sprites.as_mut());
    }
}
