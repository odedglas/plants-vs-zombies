use std::borrow::Borrow;
use std::rc::Rc;

use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement};

use crate::constants::{CANVAS_HEIGHT, CANVAS_HEIGHT_F64, CANVAS_WIDTH, CANVAS_WIDTH_F64};
use crate::log;
use crate::model::{Position, SpriteCell};
use crate::sprite::Sprite;
use crate::web_utils::{create_canvas, get_canvas_context};

pub struct Painter {
    pub canvas: HtmlCanvasElement,
    pub context: CanvasRenderingContext2d,
}

impl Painter {
    pub fn new() -> Self {
        let canvas = create_canvas(CANVAS_WIDTH, CANVAS_HEIGHT);
        let context = get_canvas_context(&canvas);

        Self { canvas, context }
    }

    pub fn clear(&self) {
        self.context
            .clear_rect(0.0, 0.0, CANVAS_WIDTH_F64, CANVAS_HEIGHT_F64);
    }

    pub fn draw_sprite(&self, sprite: &Sprite) {
        log!("Drawing {}", sprite.name);

        if let Some(image) = &sprite.image {
            let image_ref = image
                .upgrade()
                .expect("[Painter] - Cannot draw Image is not available");

            self.draw_image(
                image_ref,
                sprite.position.first().unwrap(), // TODO - Proper
                sprite.cells.first().unwrap(),    // TODO - Proper
                1.0,
            );
        }
    }

    pub fn draw_image(
        &self,
        image: Rc<HtmlImageElement>,
        pos: &Position,
        cell: &SpriteCell,
        scale: f64,
    ) {
        self.context
            .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                image.as_ref(),
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
}
