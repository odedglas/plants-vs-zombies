use std::rc::Rc;

use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement};

use crate::constants::{CANVAS_HEIGHT, CANVAS_HEIGHT_F64, CANVAS_WIDTH, CANVAS_WIDTH_F64};
use crate::log;
use crate::model::{Position, Size, SpriteCell};
use crate::sprite::{DrawingState, Sprite};
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
        let (cell, position) = DrawingState::get(sprite);

        // Draw Sprite according to it's type.
        if let Some(image) = &sprite.image {
            let image_ref = image
                .upgrade()
                .expect("[Painter] - Cannot draw Image is not available");

            self.draw_image(&image_ref, position, cell, sprite.drawing_state.scale);
        }

        // TODO TextSprite case
    }

    pub fn draw_image(
        &self,
        image: &Rc<HtmlImageElement>,
        pos: &Position,
        cell: &SpriteCell,
        scale: f64,
    ) {
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

    pub fn get_measurements_painter(size: Size) -> Painter {
        let measurements_canvas = create_canvas(size.width as u32, size.width as u32, false);

        let measurements_context = get_canvas_context(&measurements_canvas);

        Painter {
            canvas: measurements_canvas,
            context: measurements_context,
        }
    }
}
