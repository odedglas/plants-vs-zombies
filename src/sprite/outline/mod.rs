mod marching_squares;

use std::rc::Weak;

use web_sys::HtmlImageElement;

use crate::model::{Position, Size, SpriteCell};
use crate::painter::Painter;
use crate::sprite::outline::marching_squares::MarchingSquares;
use crate::sprite::{DrawingState, Sprite};

pub struct Outline;

impl Outline {
    pub fn get_outlines(sprite: &Sprite, exact: Option<bool>) -> Vec<Position> {
        let (cell, position) = DrawingState::get(sprite);
        let scale = sprite.drawing_state.scale;
        let exact_outlines = match exact {
            Some(_e) => true,
            None => false,
        };

        let size = cell.into();

        if exact_outlines {
            return Outline::get_exact_outlines(&sprite.image, cell, position, size, scale);
        }

        Outline::get_rect_outlines(position, size, scale)
    }

    pub fn get_rect_outlines(offset: &Position, size: Size, scale: f64) -> Vec<Position> {
        let scale_left = size.width * scale;
        let scale_top = size.height * scale;

        vec![
            *offset,                                                         // Top left
            Position::new(offset.top, offset.left + scale_left),             // Right top
            Position::new(offset.top + scale_top, offset.left + scale_left), // Bottom right
            Position::new(offset.top + scale_top, offset.left),              // Bottom left
        ]
    }

    pub fn get_exact_outlines(
        sprite_image: &Option<Weak<HtmlImageElement>>,
        cell: &SpriteCell,
        offset: &Position,
        size: Size,
        scale: f64,
    ) -> Vec<Position> {
        let Size { width, height } = size;
        let image_ref = sprite_image
            .as_ref()
            .unwrap()
            .upgrade()
            .expect("[Outline] - Cannot get exact outline Image is not available");

        let painter = Painter::get_measurements_painter(size);

        let image_draw_start = Position::new(0.0, 0.0);
        painter.draw_image(&image_ref, &image_draw_start, cell, scale);

        let image_data = painter
            .context
            .get_image_data(0.0, 0.0, width, height)
            .unwrap();

        MarchingSquares::new(offset.clone()).get(
            &image_data.data(),
            width as i32,
            height as i32,
        )
    }
}
