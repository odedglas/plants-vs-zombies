use crate::game::Game;
use crate::log;
use crate::model::BehaviorType;
use crate::resource_loader::ResourceKind;
use crate::sprite::{BehaviorManager, Sprite};

pub struct BattleScene;

impl BattleScene {
    pub fn start(game: &mut Game) {
        // Show Battle Background
        let mut sprites = Sprite::create_sprites(
            vec!["BattleBackground", "Button"],
            &ResourceKind::Interface,
            &game.resources,
        );

        // Show Enemies (Zombies)

        // Add Scrolling Right behavior
        BehaviorManager::toggle_behaviors(
            sprites.iter(),
            &[BehaviorType::Scroll],
            true,
            game.game_time.time,
        );

        // Once Scroll animation is a done, Show the actual PlantsChooser

        game.add_sprites(sprites.as_mut());
    }

    pub fn show_plants_chooser(game: &mut Game) {
        log!("Showing Plants choooooser");
    }
}
