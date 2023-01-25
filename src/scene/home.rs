use crate::game::Game;
use crate::log;
use crate::resource_loader::ResourceKind;
use crate::sprite::{Outline, Sprite};

pub struct HomeScene;

impl HomeScene {
    pub fn start(game: &mut Game) {
        let scene_sprites_name = vec!["SelectorBackground", "SelectorAdventureButton"];

        // Convert each scene sprites into a actual Sprite using it's corresponding Game.Resource
        let mut sprites = Sprite::create_sprites(
            scene_sprites_name,
            &ResourceKind::Interface,
            &game.resources,
        );

        // TODO calc outline on creation?
        let outline = Outline::get_outlines(sprites.get(1).unwrap(), true);

        log!("Outlines {:?}", outline);

        // Adding scene sprites into game.
        game.add_sprites(sprites.as_mut());
    }
}
