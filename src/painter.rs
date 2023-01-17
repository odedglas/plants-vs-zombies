use std::rc::Rc;

use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use crate::constants::{CANVAS_HEIGHT, CANVAS_HEIGHT_F64, CANVAS_WIDTH, CANVAS_WIDTH_F64};
use crate::log;
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

    pub fn draw_sprite(sprite: &Sprite) {
        log!("Drawing {}", sprite.name);
    }
}
