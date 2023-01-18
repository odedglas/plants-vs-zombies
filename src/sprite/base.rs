use std::rc::Weak;

use js_sys::Math;
use web_sys::HtmlImageElement;

use crate::model::{Position, SpriteCell, SpriteData};
use crate::resource_loader::{ResourceKind, Resources};

#[derive(Debug, Default)]
pub struct DrawingState {
    pub active_cell: usize,
    pub active_position: usize,
}

impl DrawingState {
    pub fn get(sprite: &Sprite) -> (Option<&SpriteCell>, Option<&Position>) {
        let cell = sprite.cells.get(sprite.drawing_state.active_cell);

        let position = sprite.position.get(sprite.drawing_state.active_position);

        return (cell, position);
    }
}

#[derive(Debug)]
pub struct Sprite {
    pub id: String,
    pub name: String,
    pub order: usize,
    pub position: Vec<Position>,
    pub cells: Vec<SpriteCell>,
    pub image: Option<Weak<HtmlImageElement>>,
    pub scale: f64,
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
    ) -> Sprite {
        Sprite {
            id: uid(name),
            name: String::from(name),
            order,
            position,
            cells,
            image,
            scale,
            drawing_state: DrawingState::default(),
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
            ..
        } = resource.data;

        Sprite::new(
            sprite_name,
            order,
            position,
            resource.cell,
            resource.image,
            scale,
        )
    }
}

pub fn uid(prefix: &str) -> String {
    format!("{}_{}", prefix, &Math::random().to_string()[2..12])
}
