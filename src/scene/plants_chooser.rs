use crate::game::Game;
use crate::log;
use crate::model::{BehaviorType, Position};
use crate::resource_loader::ResourceKind;
use crate::sprite::{BehaviorManager, Sprite};

pub struct PlantsChooser;

impl PlantsChooser {
    fn chooser_sprites() -> Vec<&'static str> {
        vec!["SeedChooserBackground", "OkButton", "ResetButton"]
    }

    pub fn show(game: &mut Game) {
        let mut sprites = Sprite::create_sprites(
            Self::chooser_sprites(),
            &ResourceKind::Interface,
            &game.resources,
        );

        Self::create_bottom_sun_score(game);

        game.add_sprites(sprites.as_mut());
    }

    pub fn clear(game: &mut Game) {
        let mut scene_sprites = vec!["SunScore"];

        scene_sprites.append(Self::chooser_sprites().as_mut());

        game.remove_sprites(scene_sprites);
    }

    fn create_bottom_sun_score(game: &mut Game) {
        let mut sun_score =
            Sprite::create_sprite("SunScore", &ResourceKind::Interface, &game.resources).remove(0);

        sun_score.position = Position::new(560.0, 138.0);

        // TODO - Dynamically bound Game sun score into this Sprite TextOverlay.

        game.add_sprite(sun_score);
    }
}
