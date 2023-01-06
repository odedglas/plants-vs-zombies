use web_sys::{HtmlCanvasElement, MouseEvent};

use crate::log;
use crate::model::GameEvent;
use crate::web_utils::create_canvas;

#[derive(Debug)]
pub struct Game {
    pub canvas: HtmlCanvasElement,
}

impl Game {
    pub fn new() -> Game {
        Game {
            canvas: create_canvas(600, 400),
        }
    }

    pub fn run(&mut self) {
        log!("Game Run iteration");
    }

    pub fn handle_event(&self, name: GameEvent, _event: MouseEvent) {
        log!("Game handling event for: {}", name.to_string())
    }
}
