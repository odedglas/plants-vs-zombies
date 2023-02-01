use crate::game::Game;
use crate::resource_loader::ResourceKind;
use crate::sprite::Sprite;

pub struct BattleScene;

impl BattleScene {
    pub fn start(game: &mut Game) {
        // Show Battle Background
        let mut sprites = Sprite::create_sprites(
            vec![
                "BattleBackground",
                "Button"
            ],
            &ResourceKind::Interface,
            &game.resources,
        );

        // Show Enemies (Zombies)

        // Add Scrolling Right behavior

        // Once Scroll animation is a done, Show the actual PlantsChooser

        game.add_sprites(sprites.as_mut());
    }
}
