use crate::game::Game;
use crate::log;
use crate::resource_loader::ResourceKind;
use crate::sprite::Sprite;

pub struct HomeScene;

impl HomeScene {
    pub fn start(game: &mut Game) {
        let scene_sprites_name = vec![
            "SelectorBackground"
        ];

        // Convert each scene sprites into a actual Sprite using it's corresponding Game.Resource
        let mut sprites: Vec<Sprite> = scene_sprites_name.iter()
            .map(|sprite_name| {
                let resource = game.resources.get_resource(
                    sprite_name,
                    ResourceKind::Interface
                );

                log!("Background Resource Cell, {:?}", resource.cell);
                log!("Background Resource data, {:?}", resource.data);

                Sprite::new(sprite_name)
            }).collect();

        // Adding scene sprites into game.
        game.add_sprites(sprites.as_mut());
    }
}