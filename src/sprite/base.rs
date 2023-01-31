use std::cell::{RefCell, RefMut};
use std::rc::Weak;

use js_sys::Math;
use web_sys::HtmlImageElement;

use crate::log;
use crate::model::{BehaviorData, Position, SpriteCell, SpriteData};
use crate::resource_loader::{Resource, ResourceKind, Resources};
use crate::sprite::behavior::{Behavior, BehaviorManager};
use crate::sprite::drawing_state::DrawingState;
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
}

impl Sprite {
    pub fn new(
        name: &str,
        order: usize,
        position: Position,
        cells: Vec<SpriteCell>,
        image: Option<Weak<HtmlImageElement>>,
        scale: f64,
        behaviors: &Vec<BehaviorData>,
        exact_outlines: bool,
    ) -> Sprite {
        let sprite_behaviors = RefCell::new(
            behaviors
                .iter()
                .map(|behavior_data| BehaviorManager::create(&behavior_data))
                .collect(),
        );

        let mut sprite = Sprite {
            id: uid(name),
            name: String::from(name),
            order,
            position,
            image,
            drawing_state: DrawingState::new(cells, scale),
            outlines: vec![],
            behaviors: sprite_behaviors,
        };

        sprite.outlines = Outline::get_outlines(&sprite, exact_outlines);

        sprite
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
            order,
            scale,
            behaviors,
            exact_outlines,
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
                    resource.cell,
                    resource.image,
                    scale,
                    &behaviors,
                    exact_outlines,
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

            if let Some(position) = mutation.position {
                log!("TODO - Sprite position Changed")
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
