use std::cell::{RefCell, RefMut};
use std::rc::Weak;

use js_sys::Math;
use web_sys::HtmlImageElement;

use crate::board::{Board, BoardLocation};
use crate::location_builder::LocationBuilder;
use crate::log;
use crate::model::{
    BehaviorData, BehaviorType, CollisionMargin, Dimensions, Position, SpriteCell, SpriteData,
    SpriteType, TextOverlayData,
};
use crate::resource_loader::{Resource, ResourceKind, Resources};
use crate::sprite::attack_state::AttackState;
use crate::sprite::behavior::{Animate, Behavior, BehaviorManager, Collision};
use crate::sprite::drawing_state::DrawingState;
use crate::sprite::text_overlay::TextOverlay;
use crate::sprite::{Outline, SpriteMutation};
use crate::web_utils::window_time;

pub struct Sprite {
    pub id: String,
    pub name: String,
    pub order: usize,
    pub position: Position,
    pub origin_position: Position,
    pub board_location: BoardLocation,
    pub outlines: Vec<Position>,
    pub behaviors: RefCell<Vec<Box<dyn Behavior>>>,
    pub image: Option<Weak<HtmlImageElement>>,
    pub drawing_state: DrawingState,
    pub attack_state: AttackState,
    pub text_overlay: Option<TextOverlay>,
    pub sprite_type: SpriteType,
    pub visible: bool,
}

impl Sprite {
    pub fn new(
        name: &str,
        order: usize,
        position: Position,
        draw_offset: Position,
        cells: Vec<SpriteCell>,
        swap_cells: Vec<Vec<SpriteCell>>,
        image: Option<Weak<HtmlImageElement>>,
        scale: f64,
        behaviors: &Vec<BehaviorData>,
        exact_outlines: bool,
        text_overlay_data: &Option<TextOverlayData>,
        kind: ResourceKind,
        life: f64,
        damage: f64,
    ) -> Sprite {
        let id = uid(name);
        let sprite_type = SpriteType::from_kind(&kind);

        let sprite_behaviors = RefCell::new(
            behaviors
                .iter()
                .map(|behavior_data| BehaviorManager::create(&behavior_data, id.clone()))
                .collect(),
        );

        let mut sprite = Sprite {
            id,
            name: String::from(name),
            order,
            position,
            origin_position: position,
            board_location: BoardLocation::new(0, 0),
            image,
            drawing_state: DrawingState::new(cells, swap_cells, scale, draw_offset),
            attack_state: AttackState::new(life, damage),
            outlines: vec![],
            behaviors: sprite_behaviors,
            text_overlay: None,
            sprite_type,
            visible: true,
        };

        sprite.text_overlay = match text_overlay_data {
            Some(data) => Some(TextOverlay::new(data, &sprite)),
            None => None,
        };

        sprite.update_board_location();
        sprite.update_outlines(exact_outlines);

        sprite
    }

    pub fn dimensions(&self) -> Dimensions {
        let active_cell = DrawingState::get_active_cell(&self);

        return SpriteCell {
            left: self.position.left,
            top: self.position.top,
            width: active_cell.width,
            height: active_cell.height,
        };
    }

    pub fn update_position(&mut self, position: Position) {
        self.position = position;

        self.update_board_location();
        self.update_outlines(false);
    }

    pub fn update_outlines(&mut self, exact_outlines: bool) {
        self.outlines = Outline::get_outlines(&self, exact_outlines);
    }

    pub fn update_board_location(&mut self) {
        let sprite_cell = DrawingState::get_active_cell(self);

        let sprite_center = Position::new(
            self.position.top + sprite_cell.height / 2.0,
            self.position.left + sprite_cell.width / 2.0,
        );

        self.board_location = Board::get_board_location(&sprite_center);
    }

    pub fn update_swap_cell(&mut self, swap_index: i32) {
        let current_cell = DrawingState::get_active_cell(self).clone();

        if swap_index >= 0 {
            self.drawing_state.swap(swap_index as usize);
        } else {
            self.drawing_state.reset_swap();
        }

        // After swap, We need to re-place the sprite over the cell
        let new_cell = DrawingState::get_active_cell(self);
        let swapped_position =
            LocationBuilder::align_sprite_after_cells_swap(&self, new_cell, &current_cell);

        self.update_position(swapped_position);
    }

    pub fn create_sprites(
        sprite_names: Vec<&str>,
        kind: &ResourceKind,
        resources: &Resources,
    ) -> Vec<Sprite> {
        return sprite_names
            .iter()
            .flat_map(|sprite_name| Sprite::create_sprite(sprite_name, kind, resources))
            .collect();
    }

    /// Creates a Sprite by a given name and kind.
    pub fn create_sprite(
        sprite_name: &str,
        kind: &ResourceKind,
        resources: &Resources,
    ) -> Vec<Sprite> {
        let Resource { data, .. } = resources.get_resource(sprite_name, kind);

        let SpriteData {
            position,
            draw_offset,
            order,
            scale,
            behaviors,
            exact_outlines,
            text_overlay,
            life,
            damage,
            swap_cells,
            ..
        } = data;

        // Map each position into it's own Sprite.
        position
            .iter()
            .map(|position| {
                let resource = resources.get_resource(sprite_name, kind);

                let swap_cells = swap_cells
                    .iter()
                    .map(|cell_name| resources.get_cell(cell_name, kind))
                    .collect::<Vec<Vec<SpriteCell>>>();

                Sprite::new(
                    sprite_name,
                    order,
                    *position,
                    draw_offset,
                    resource.cell,
                    swap_cells,
                    resource.image,
                    scale,
                    &behaviors,
                    exact_outlines,
                    &text_overlay,
                    kind.clone(),
                    life,
                    damage,
                )
            })
            .collect()
    }

    pub fn apply_mutation(&mut self, mutations: Vec<SpriteMutation>) {
        mutations.iter().for_each(|mutation| {
            if let Some(hovered) = mutation.hovered {
                self.drawing_state.hover(hovered);
            }

            if let Some(_) = mutation.cycle_cells {
                self.drawing_state.cycle_cells();
            }

            if let Some(offset) = mutation.offset {
                self.drawing_state.offset = offset;
            }

            if let Some(position) = mutation.position {
                self.update_position(position);
            }

            if let Some(visible) = mutation.visible {
                self.visible = visible;
            }

            if let Some(alpha) = mutation.alpha {
                self.drawing_state.alpha = alpha;
            }

            if let Some(damage) = mutation.damage {
                self.attack_state.take_damage(damage);
            }

            if let Some(swap_index) = mutation.swap {
                self.update_swap_cell(swap_index);
            }

            if let Some(walking) = mutation.walking {
                self.toggle_walking(walking);
            }

            if let Some(mute) = mutation.mute {
                self.attack_state.mute(!mute);
                self.toggle_walking(!mute);
            }

            if let Some(_) = mutation.stop_animate {
                let animate = BehaviorManager::get_sprite_behavior(self, BehaviorType::Animate)
                    .as_any()
                    .downcast_mut::<Animate>()
                    .unwrap();

                animate.set_max_cycles(1);
            }
        });
    }

    pub fn mutable_behaviors(&self) -> RefMut<'_, Vec<Box<dyn Behavior>>> {
        self.behaviors.borrow_mut()
    }

    pub fn get_collision(&self) -> Option<CollisionMargin> {
        let mut behaviors = self.behaviors.borrow_mut();

        let collision = behaviors
            .iter_mut()
            .find(|behavior| behavior.name() == BehaviorType::Collision);

        match collision {
            None => None,
            Some(collision) => Some(
                collision
                    .as_any()
                    .downcast_mut::<Collision>()
                    .unwrap()
                    .margin,
            ),
        }
    }

    pub fn toggle_walking(&mut self, walking: bool) {
        BehaviorManager::toggle_sprite_behaviors(self, &[BehaviorType::Walk], walking, window_time())
    }
}

pub fn uid(prefix: &str) -> String {
    format!("{}_{}", prefix, &Math::random().to_string()[2..12])
}
