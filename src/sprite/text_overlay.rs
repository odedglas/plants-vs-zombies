use web_sys::TextMetrics;

use crate::log;
use crate::model::{Position, Size, SpriteCell, TextOverlayData};
use crate::painter::Painter;

#[derive(Debug)]
pub struct TextOverlay {
    pub offset: Option<Position>,
    pub text: String,
    pub size: usize,
    pub position: Option<Position>,
}

impl TextOverlay {
    pub fn new(data: &TextOverlayData, source_cell: &SpriteCell, source_position: &Position) -> Self {
        let mut overlay = TextOverlay {
            text: data.text.clone(),
            size: data.size,
            offset: data.offset,
            position: None,
        };

        overlay.calculate_text_position(source_cell, source_position);

        overlay
    }

    fn calculate_text_position(&mut self, source_cell: &SpriteCell, source_position: &Position) {
        // Measure current text by it's size, extract a rect of it
        let measure_painter = Painter::get_measurements_painter(Size::new(200.0, 200.0));

        let font_size = format!("{}px Kavivanar", self.size);

        measure_painter.context.set_font(&font_size);
        measure_painter.context.set_text_baseline("top");

        log!("Cell size {:?}", source_cell);
        let text_metrics: TextMetrics = measure_painter.context.measure_text(&self.text).unwrap();
        let text_size: Size = text_metrics.into();
        log!("Text Size{} / {}", text_size.height, text_size.width);

        // Position current Text rect within source cell.
        let center_x = source_position.left + (source_cell.width - text_size.width) / 2.0;
        let center_y = source_position.top + (source_cell.height - text_size.height) / 2.0;

        self.position = Some(Position::new(center_y, center_x));

        log!("Final position at {}/{}", center_x, center_y)
    }
}
