use crate::game::Game;
use crate::model::BehaviorType;
use crate::resource_loader::ResourceKind;
use crate::sprite::{BehaviorManager, Sprite};

pub struct HomeScene;

impl HomeScene {
    pub fn home_sprites() -> Vec<&'static str> {
        vec![
            "SelectorBackground",
            "SelectorAdventureShadow",
            "SelectorSurvivalShadow",
            "SelectorChallengeShadow",
            "SelectorWoodSign1",
            "SelectorWoodSign2",
            "SelectorWoodSign3",
            "SelectorAdventureButton",
            "SelectorSurvivalButton",
            "SelectorChallengeButton",
        ]
    }

    pub fn start(game: &mut Game) {
        let scene_sprites_name = HomeScene::home_sprites();

        // Convert each scene sprites into a actual Sprite using it's corresponding Game.Resource
        let mut sprites = Sprite::create_sprites(
            scene_sprites_name,
            &ResourceKind::Interface,
            &game.resources,
        );

        // Adding scene sprites into game.
        game.add_sprites(sprites.as_mut());
    }

    pub fn show_zombie_hand(game: &mut Game) {
        let mut zombie_hand = Sprite::create_sprite(
            "SelectorZombieHand",
            &ResourceKind::Interface,
            &game.resources,
        );

        // Activates zombie hand animation Cycle.
        BehaviorManager::toggle_behaviors(
            &zombie_hand,
            &[BehaviorType::Animate],
            true,
            game.game_time.time,
        );

        game.add_sprites(zombie_hand.as_mut());
    }
}
