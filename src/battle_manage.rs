use itertools::Itertools;

use crate::game::Game;
use crate::log;
use crate::model::{BehaviorType, SpriteType};
use crate::sprite::Sprite;

pub struct BattleManager;

struct CollisionMutation {
    target_id: String,
    life_deduction: Option<usize>,
}

impl CollisionMutation {
    pub fn new(id: &String) -> Self {
        CollisionMutation {
            target_id: String::from(id),
            life_deduction: None,
        }
    }
}

// Activate their `on_collision` hook
// Manage fight life

impl BattleManager {
    pub fn manage_fight(game: &mut Game) {
        let mut mutations = Self::collect_collision_mutations(game);

        game.sprites.iter_mut().for_each(|sprite| {
            let mutation = mutations
                .iter()
                .find(|mutation| mutation.target_id == sprite.id);

            if let Some(mutation) = mutation {
                log!("Activation mutation! on {}", mutation.target_id)

                // Apply sprite mutation

                // Trigger on_collide?
            }
        });
    }

    fn collect_collision_mutations(game: &mut Game) -> Vec<CollisionMutation> {
        let mut mutations: Vec<CollisionMutation> = vec![];

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
                    let candidates = others
                        .iter()
                        .filter(|other| Self::can_collide(sprite, other))
                        .collect::<Vec<&&Sprite>>();

                    // For each candidate, Check if collided
                    let collided_candidate = candidates
                        .iter()
                        .find(|candidate| Self::has_collision(sprite, candidate));

                    if let Some(collided_sprite) = collided_candidate {
                        mutations.push(CollisionMutation::new(&collided_sprite.id));
                    }
                });
            });

        mutations
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
            _ => SpriteType::Meta,
        };

        &target_type == &other.sprite_type
    }

    fn has_collision(sprite: &Sprite, other: &Sprite) -> bool {
        // TODO - Make it real
        sprite.position.left > other.position.left
    }
}
