use crate::location_builder::LocationBuilder;
use crate::model::{LocationType, Position, TextOverlayData};
use crate::painter::Painter;
use crate::sprite::Sprite;

#[derive(Debug)]
pub struct TextOverlay {
    pub offset: Option<Position>,
    pub text: String,
    pub size: usize,
    pub position: Option<Position>,
    pub location_type: LocationType,
    pub color: Option<String>,
}

impl TextOverlay {
    pub fn new(data: &TextOverlayData, source_sprite: &Sprite) -> Self {
        let mut overlay = TextOverlay {
            text: data.text.clone(),
            size: data.size,
            offset: data.offset,
            position: None,
            location_type: data.location_type,
            color: data.color.clone(),
        };

        overlay.calculate_text_position(source_sprite);

        overlay
    }

    fn calculate_text_position(&mut self, source_sprite: &Sprite) {
        // Measure current text by it's size, extract a rect of it
        let text_size = Painter::measure_text(&self.text, self.size);

        // Placing text at the center of the given source sprite.
        self.position = Some(LocationBuilder::locate_text_overlay(
            source_sprite,
            text_size,
            &self.location_type,
        ));
    }
}
