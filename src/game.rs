use web_sys::{HtmlCanvasElement, MouseEvent};

use crate::log;
use crate::model::GameEvent;
use crate::resource_loader::Resources;
use crate::web_utils::create_canvas;

pub struct Game {
    pub resources: Resources,
    pub canvas: HtmlCanvasElement,
}

impl Game {
    pub fn new() -> Game {
        Game {
            canvas: create_canvas(600, 400),
            resources: Resources::new(),
        }
    }

    pub fn init(&mut self, resources: Resources) {
        self.resources = resources;
    }

    pub fn run(&mut self) {
        log!("Game Run iteration");
    }

    pub fn handle_event(&self, name: GameEvent, _event: MouseEvent) {
        log!("Game handling event for: {}", name.to_string())
    }
}
