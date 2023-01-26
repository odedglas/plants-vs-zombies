use std::rc::Weak;

use js_sys::Math;
use web_sys::HtmlImageElement;

use crate::log;
use crate::model::{BehaviorData, Position, SpriteCell, SpriteData};
use crate::resource_loader::{ResourceKind, Resources};
use crate::sprite::behavior::{Behavior, BehaviorManager};
use crate::sprite::SpriteMutation;

#[derive(Debug, Default)]
pub struct DrawingState {
    pub active_cell: usize,
    pub active_position: usize,
    pub scale: f64,
}

impl DrawingState {
    pub fn new(scale: f64) -> Self {
        Self {
            scale,
            ..DrawingState::default()
        }
    }

    pub fn get(sprite: &Sprite) -> (&SpriteCell, &Position) {
        let cell = sprite
            .cells
            .get(sprite.drawing_state.active_cell)
            .expect(&format!(
                "[Sprite] Cannot get drawing state cell of {}",
                sprite.name
            ));

        let position = sprite
            .position
            .get(sprite.drawing_state.active_position)
            .expect(&format!(
                "[Sprite] Cannot get drawing state position of {}",
                sprite.name
            ));

        return (cell, position);
    }
}

pub struct Sprite {
    pub id: String,
    pub name: String,
    pub order: usize,
    pub position: Vec<Position>,
    pub outlines: Vec<Position>,
    pub behaviors: Vec<Box<dyn Behavior>>,
    pub cells: Vec<SpriteCell>,
    pub image: Option<Weak<HtmlImageElement>>,
    pub drawing_state: DrawingState,
}

impl Sprite {
    pub fn new(
        name: &str,
        order: usize,
        position: Vec<Position>,
        cells: Vec<SpriteCell>,
        image: Option<Weak<HtmlImageElement>>,
        scale: f64,
        behaviors: Vec<BehaviorData>,
    ) -> Sprite {
        let id = uid(name);
        let sprite_behaviors = behaviors
            .iter()
            .map(|behavior_data| {
                log!("Adding behaviour for {} / {:?}", name, behavior_data.name);
                BehaviorManager::create(&id, &behavior_data.name)
            })
            .collect();

        Sprite {
            id,
            name: String::from(name),
            order,
            position,
            cells,
            image,
            drawing_state: DrawingState::new(scale),
            outlines: vec![],
            behaviors: sprite_behaviors,
        }
    }

    pub fn create_sprites(
        sprite_names: Vec<&str>,
        kind: &ResourceKind,
        resources: &Resources,
    ) -> Vec<Sprite> {
        return sprite_names
            .iter()
            .map(|sprite_name| Sprite::create_sprite(sprite_name, kind, resources))
            .collect();
    }

    /// Creates a Sprite by a given name and kind.
    pub fn create_sprite(sprite_name: &str, kind: &ResourceKind, resources: &Resources) -> Sprite {
        let resource = resources.get_resource(sprite_name, kind);

        let SpriteData {
            position,
            order,
            scale,
            behaviors,
            ..
        } = resource.data;

        Sprite::new(
            sprite_name,
            order,
            position,
            resource.cell,
            resource.image,
            scale,
            behaviors,
        )
    }

    pub fn apply_mutation(&mut self, mutations: Vec<SpriteMutation>) {
        mutations.iter().for_each(|mutation| {
            log!("Apply sprite mutation {}", self.id);
            if let Some(hovered) = mutation.hovered {
                log!("Sprite Hovered!");
            }

            if let Some(clicked) = mutation.clicked {
                log!("Sprite clicked")
            }

            if let Some(position) = mutation.position {
                log!("Sprite position Changed")
            }
        });
    }
}

pub fn uid(prefix: &str) -> String {
    format!("{}_{}", prefix, &Math::random().to_string()[2..12])
}
