use itertools::Itertools;

use crate::game::Game;
use crate::model::{BehaviorType, CollisionMargin, SpriteType};
use crate::sprite::{
    BehaviorManager, Collision, CollisionState, DrawingState, Sprite,
};

struct CollisionMutation {
    attacking_id: String,
    target_id: String,
    damage: f64,
}

impl CollisionMutation {
    pub fn new(attacking_id: &String, target_id: &String, damage: f64) -> Self {
        CollisionMutation {
            attacking_id: String::from(attacking_id),
            target_id: String::from(target_id),
            damage,
        }
    }
}

pub struct BattleManager;

impl BattleManager {
    pub fn manage_fight(game: &mut Game) {
        let mutations = Self::collect_collision_mutations(game);

        Self::flag_collision_state(game, mutations);
    }

    fn flag_collision_state(game: &mut Game, mutations: Vec<CollisionMutation>) {
        game.sprites
            .iter_mut()
            .filter(|sprite| sprite.get_collision().is_some())
            .for_each(|sprite| {
                let sprite_id = sprite.id.clone();

                let mutations = mutations
                    .iter()
                    .filter(|mutation| {
                        mutation.target_id == sprite.id || mutation.attacking_id == sprite.id
                    })
                    .collect::<Vec<&CollisionMutation>>();

                let collision = BehaviorManager::get_sprite_behavior(sprite, BehaviorType::Collision)
                    .as_any()
                    .downcast_mut::<Collision>()
                    .unwrap();

                if mutations.len() > 0 {
                    mutations.iter().for_each(|mutation| {
                        if mutation.attacking_id == sprite_id {
                            collision.state = CollisionState::Attacking;
                        }

                        if mutation.target_id == sprite_id {
                            collision.state = CollisionState::TakingDamage(mutation.damage);
                        }
                    })
                } else {
                    collision.state = CollisionState::None;
                }
            });
    }

    fn collect_collision_mutations(game: &mut Game) -> Vec<CollisionMutation> {
        let mut mutations: Vec<CollisionMutation> = vec![];

        game.sprites
            .iter()
            .sorted_by(|a, b| a.board_location.row.cmp(&b.board_location.row))
            .filter(|sprite| sprite.visible && Self::has_collision_behavior(sprite))
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
                        mutations.push(CollisionMutation::new(
                            &sprite.id,
                            &collided_sprite.id,
                            sprite.attack_state.get_damage(),
                        ));
                    }
                });
            });

        mutations
    }

    pub fn has_collision_behavior(sprite: &Sprite) -> bool {
        sprite
            .behaviors
            .borrow()
            .iter()
            .find(|behavior| {
                behavior.name() == BehaviorType::Collision && behavior.is_running() == true
            })
            .is_some()
    }

    fn can_collide(sprite: &Sprite, other: &Sprite) -> bool {
        let source_type = &sprite.sprite_type;

        let target_type = match source_type {
            SpriteType::Zombie => SpriteType::Plant,
            SpriteType::Bullet => SpriteType::Zombie,
            SpriteType::Plant => SpriteType::Bullet,
            _ => SpriteType::Meta,
        };

        &target_type == &other.sprite_type
    }

    fn has_collision(sprite: &Sprite, target: &Sprite) -> bool {
        let collision = sprite.get_collision().unwrap_or(CollisionMargin::default());

        let collision_left = sprite.position.left + collision.left as f64;

        let target_cell = DrawingState::get_active_cell(target);

        collision_left >= target.position.left
            && collision_left <= target.position.left + target_cell.width
    }
}
