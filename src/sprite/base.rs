use std::rc::Weak;

use js_sys::Math;
use web_sys::HtmlImageElement;

use crate::model::{Position, SpriteCell, SpriteData};
use crate::resource_loader::{ResourceKind, Resources};

#[derive(Debug)]
pub struct Sprite {
    pub id: String,
    pub name: String,
    pub order: usize,
    pub position: Vec<Position>,
    pub cells: Vec<SpriteCell>,
    pub image: Option<Weak<HtmlImageElement>>,
}

impl Sprite {
    pub fn new(
        name: &str,
        order: usize,
        position: Vec<Position>,
        cells: Vec<SpriteCell>,
        image: Option<Weak<HtmlImageElement>>,
    ) -> Sprite {
        Sprite {
            id: uid(name),
            name: String::from(name),
            order,
            position,
            cells,
            image,
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
            position, order, ..
        } = resource.data;

        Sprite::new(sprite_name, order, position, resource.cell, resource.image)
    }
}

pub fn uid(prefix: &str) -> String {
    format!("{}_{}", prefix, &Math::random().to_string()[2..12])
}
