use crate::game::Game;
use crate::location_builder::LocationBuilder;
use crate::model::{Position, Size, SpriteType};
use crate::resource_loader::ResourceKind;
use crate::sprite::Sprite;

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

        let chooser_background_offset = &sprites.first().unwrap().position;

        Self::build_cards_layout(game, chooser_background_offset);
        Self::create_bottom_sun_score(game);

        game.add_sprites(sprites.as_mut());
    }

    pub fn clear(game: &mut Game) {
        let mut scene_sprites = vec!["SunScore"];
        scene_sprites.append(Self::chooser_sprites().as_mut());

        game.remove_sprites_by_name(scene_sprites);
        game.remove_sprites_by_type(&SpriteType::Seed)
    }

    pub fn reset_selection(game: &mut Game) {
        game.get_sprites_by_type(&SpriteType::Seed)
            .iter_mut()
            .for_each(|seed| {
                seed.drawing_state.hover(false);
            });

        game.remove_sprites_by_type(&SpriteType::Card);
    }

    fn create_bottom_sun_score(game: &mut Game) {
        let mut sun_score =
            Sprite::create_sprite("SunScore", &ResourceKind::Interface, &game.resources).remove(0);

        sun_score.position = Position::new(560.0, 98.0);

        // TODO - Dynamically bound Game sun score into this Sprite TextOverlay.

        game.add_sprite(sun_score);
    }

    fn build_cards_layout(game: &mut Game, offset: &Position) {
        let card_scale = 0.725;
        let positions = LocationBuilder::create_row_layout(
            &Position::new(offset.top + 34.0, offset.left + 14.0),
            game.state.get_level().plant_cards.len(),
            6,
            Size::new(100.0 * card_scale, 60.0 * card_scale),
        );

        let mut cards = game
            .state
            .get_level()
            .plant_cards
            .iter()
            .enumerate()
            .flat_map(|(index, card_name)| {
                let mut card_sprite =
                    Sprite::create_sprite(card_name.trim(), &ResourceKind::Card, &game.resources);

                card_sprite.iter_mut().for_each(|card| {
                    card.update_position(positions[index]);
                    card.sprite_type = SpriteType::Seed;
                });

                card_sprite
            })
            .collect::<Vec<Sprite>>();

        game.add_sprites(cards.as_mut());
    }
}
