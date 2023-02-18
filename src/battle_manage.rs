use itertools::Itertools;

use crate::game::Game;
use crate::log;
use crate::model::SpriteType;
use crate::sprite::Sprite;

pub struct BattleManager;

impl BattleManager {
    pub fn manage_fight(game: &mut Game) {
        game.sprites
            .iter_mut()
            .sorted_by(|a, b| a.board_location.row.cmp(&b.board_location.row))
            .filter(|sprite| {
                sprite.sprite_type == SpriteType::Bullet || sprite.sprite_type == SpriteType::Zombie
            }) // TODO -Filter with has_collision
            .group_by(|sprite| sprite.board_location.row)
            .into_iter()
            .map(|(_, items)| items.collect::<Vec<&mut Sprite>>())
            .for_each(|group| group.iter().for_each(|s| log!("Procssing {}", s.id)));
    }
}
