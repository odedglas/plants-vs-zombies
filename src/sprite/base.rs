use std::cell::{RefCell, RefMut};
use std::rc::Weak;

use js_sys::Math;
use web_sys::HtmlImageElement;

use crate::model::{
    BehaviorData, Dimensions, Position, SpriteCell, SpriteData, SpriteType, TextOverlayData,
};
use crate::resource_loader::{Resource, ResourceKind, Resources};
use crate::sprite::behavior::{Behavior, BehaviorManager};
use crate::sprite::drawing_state::DrawingState;
use crate::sprite::text_overlay::TextOverlay;
use crate::sprite::{Outline, SpriteMutation};

pub struct Sprite {
    pub id: String,
    pub name: String,
    pub order: usize,
    pub position: Position,
    pub outlines: Vec<Position>,
    pub behaviors: RefCell<Vec<Box<dyn Behavior>>>,
    pub image: Option<Weak<HtmlImageElement>>,
    pub drawing_state: DrawingState,
    pub text_overlay: Option<TextOverlay>,
    pub sprite_type: SpriteType,
}

impl Sprite {
    pub fn new(
        name: &str,
        order: usize,
        position: Position,
        draw_offset: Position,
        cells: Vec<SpriteCell>,
        image: Option<Weak<HtmlImageElement>>,
        scale: f64,
        behaviors: &Vec<BehaviorData>,
        exact_outlines: bool,
        text_overlay_data: &Option<TextOverlayData>,
        kind: ResourceKind,
    ) -> Sprite {
        let id = uid(name);
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
            image,
            drawing_state: DrawingState::new(cells, scale, draw_offset),
            outlines: vec![],
            behaviors: sprite_behaviors,
            text_overlay: None,
            sprite_type: SpriteType::from_kind(&kind),
        };

        sprite.text_overlay = match text_overlay_data {
            Some(data) => Some(TextOverlay::new(data, &sprite)),
            None => None,
        };

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
        self.update_outlines(false);
    }

    pub fn update_outlines(&mut self, exact_outlines: bool) {
        self.outlines = Outline::get_outlines(&self, exact_outlines);
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
            ..
        } = data;

        // Map each position into it's own Sprite.
        position
            .iter()
            .map(|position| {
                let resource = resources.get_resource(sprite_name, kind);
                Sprite::new(
                    sprite_name,
                    order,
                    *position,
                    draw_offset,
                    resource.cell,
                    resource.image,
                    scale,
                    &behaviors,
                    exact_outlines,
                    &text_overlay,
                    kind.clone(),
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
        });
    }

    pub fn mutable_behaviors(&self) -> RefMut<'_, Vec<Box<dyn Behavior>>> {
        self.behaviors.borrow_mut()
    }
}

pub fn uid(prefix: &str) -> String {
    format!("{}_{}", prefix, &Math::random().to_string()[2..12])
}
