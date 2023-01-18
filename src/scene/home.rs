use crate::game::Game;
use crate::resource_loader::ResourceKind;
use crate::sprite::Sprite;

pub struct HomeScene;

impl HomeScene {
    pub fn start(game: &mut Game) {
        let scene_sprites_name = vec![
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
        ];

        // Convert each scene sprites into a actual Sprite using it's corresponding Game.Resource
        let mut sprites = Sprite::create_sprites(
            scene_sprites_name,
            &ResourceKind::Interface,
            &game.resources,
        );

        // Adding scene sprites into game.
        game.add_sprites(sprites.as_mut());
    }
}
