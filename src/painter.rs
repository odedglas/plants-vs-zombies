use std::rc::Rc;

use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement, TextMetrics};

use crate::constants::{CANVAS_HEIGHT, CANVAS_HEIGHT_F64, CANVAS_WIDTH, CANVAS_WIDTH_F64};
use crate::model::{Position, Size, SpriteCell};
use crate::sprite::{DrawingState, Sprite, TextOverlay};
use crate::web_utils::{create_canvas, get_canvas_context};

pub struct Painter {
    pub canvas: HtmlCanvasElement,
    pub context: CanvasRenderingContext2d,
}

impl Painter {
    pub fn new() -> Self {
        let canvas = create_canvas(CANVAS_WIDTH, CANVAS_HEIGHT, true);
        let context = get_canvas_context(&canvas);

        Self { canvas, context }
    }

    pub fn clear(&self) {
        self.context
            .clear_rect(0.0, 0.0, CANVAS_WIDTH_F64, CANVAS_HEIGHT_F64);
    }

    pub fn draw_sprite(&self, sprite: &Sprite) {
        let cell = DrawingState::get_active_cell(sprite);

        // Draw Sprite according to it's type.
        if let Some(image) = &sprite.image {
            let image_ref = image
                .upgrade()
                .expect("[Painter] - Cannot draw Image is not available");

            self.draw_image(
                &image_ref,
                &sprite.position,
                &sprite.drawing_state.offset,
                cell,
                sprite.drawing_state.scale,
                sprite.drawing_state.alpha,
            );
        }

        if let Some(text_overlay) = &sprite.text_overlay {
            self.draw_text_overlay(text_overlay);
        }
    }

    pub fn draw_image(
        &self,
        image: &Rc<HtmlImageElement>,
        pos: &Position,
        offset: &Position,
        cell: &SpriteCell,
        scale: f64,
        alpha: f64,
    ) {
        // Setting translate if defined, which will cause a "partial image" view.
        self.context.translate(-offset.left, -offset.top).unwrap();
        self.context.set_global_alpha(alpha);

        self.context
            .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                image,
                cell.left,
                cell.top,
                cell.width,
                cell.height,
                pos.left,
                pos.top,
                cell.width * scale,
                cell.height * scale,
            )
            .unwrap();

        // Restoring translate
        self.context.translate(offset.left, offset.top).unwrap();
    }

    pub fn draw_text_overlay(&self, text_overlay: &TextOverlay) {
        self.context.save();
        self.set_text_styles(text_overlay.size);

        if let Some(color) = &text_overlay.color {
            self.context.set_fill_style(&color.into());
        }

        let position = &text_overlay.position.unwrap();
        let offset = &text_overlay.offset.unwrap_or(Position::default());

        self.context
            .fill_text(
                &text_overlay.text,
                position.left + offset.left,
                position.top + offset.top,
            )
            .unwrap();

        self.context.restore();
    }

    pub fn draw_line(&self, start: &Position, to: &Position) {
        self.context.save();
        self.context.begin_path();

        self.context.begin_path();

        self.context.move_to(start.left, start.top);
        self.context.line_to(to.left, to.top);
        self.context.set_line_width(0.3);

        self.context.close_path();

        self.context.stroke();

        self.context.restore();
    }

    pub fn measure_text(text: &str, size: usize) -> Size {
        let measure_painter = Painter::get_measurements_painter(Size::new(200.0, 200.0));

        measure_painter.set_text_styles(size);

        let text_metrics: TextMetrics = measure_painter.context.measure_text(&text).unwrap();
        let text_size: Size = text_metrics.into();

        text_size
    }

    pub fn in_path(
        outline: &Vec<Position>,
        point: &Position,
        context: &CanvasRenderingContext2d,
    ) -> bool {
        if outline.is_empty() {
            return false;
        }

        context.save();
        context.set_global_alpha(0.0);
        context.begin_path();

        let first = outline.get(0).unwrap();
        context.move_to(first.left, first.top);

        // Draw outline within context
        outline
            .iter()
            .skip(1)
            .for_each(|path| context.line_to(path.left, path.top));

        context.close_path();
        context.stroke();

        context.restore();

        // Check rather if point is within that shape.
        context.is_point_in_path_with_f64(point.left, point.top)
    }

    pub fn set_text_styles(&self, size: usize) {
        let font_size = format!("{}px Kavivanar", size);

        self.context.set_font(&font_size);
        self.context.set_fill_style(&"white".into());
        self.context.set_text_baseline("top");
    }

    pub fn get_measurements_painter(size: Size) -> Painter {
        let measurements_canvas = create_canvas(size.width as u32, size.width as u32, false);

        let measurements_context = get_canvas_context(&measurements_canvas);

        Painter {
            canvas: measurements_canvas,
            context: measurements_context,
        }
    }
}
