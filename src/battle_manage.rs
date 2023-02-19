use itertools::Itertools;

use crate::game::Game;
use crate::log;
use crate::model::{BehaviorType, SpriteType};
use crate::sprite::Sprite;

pub struct BattleManager;

// Activate their `on_collision` hook
// Manage fight life

impl BattleManager {
    pub fn manage_fight(game: &mut Game) {
        game.sprites
            .iter()
            .sorted_by(|a, b| a.board_location.row.cmp(&b.board_location.row))
            .filter(|sprite| Self::has_collision_behavior(sprite))
            .group_by(|sprite| sprite.board_location.row)
            .into_iter()
            .map(|(_, items)| items.collect::<Vec<&Sprite>>())
            .for_each(|group| {
                let others = group.to_vec();

                group.into_iter().for_each(|sprite| {
                    // For each given sprite within the group, finding respective collision candidates
                    let candidates = others.iter()
                        .filter(|other| Self::can_collide(sprite, other))
                        .collect::<Vec<&&Sprite>>();

                    // For each candidate, Check if collided
                    let collided_candidate = candidates.iter()
                        .find(|candidate| Self::has_collision(sprite, candidate));


                    log!("Procssing {} / {} ", sprite.id, candidates.len());

                    if let Some(collided_sprite) = collided_candidate {
                        log!("Found collided Sprite {}", collided_sprite.id)
                    }
                });
            });
    }

    fn has_collision_behavior(sprite: &&Sprite) -> bool {
        let behaviors = sprite.behaviors.borrow();

        behaviors
            .iter()
            .find(|sprite_behavior| BehaviorType::Collision == sprite_behavior.name())
            .is_some()
    }

    fn can_collide(sprite: &Sprite, other: &Sprite) -> bool {
        let source_type = &sprite.sprite_type;

        let target_type = match source_type {
            SpriteType::Zombie => SpriteType::Plant,
            SpriteType::Bullet => SpriteType::Zombie,
            _ => SpriteType::Meta
        };

        &target_type == &other.sprite_type
    }

    fn has_collision(sprite: &Sprite, other: &Sprite) -> bool {
        false
    }

}
