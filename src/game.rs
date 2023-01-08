use web_sys::{HtmlCanvasElement, MouseEvent};

use crate::fps::Fps;
use crate::log;
use crate::model::GameEvent;
use crate::resource_loader::Resources;
use crate::timers::GameTime;
use crate::web_utils::create_canvas;

pub struct Game {
    pub resources: Resources,
    pub canvas: HtmlCanvasElement,

    game_time: GameTime,
    fps: Fps,
}

impl Game {
    pub fn new() -> Game {
        Game {
            canvas: create_canvas(600, 400),
            resources: Resources::new(),
            game_time: GameTime::new(),
            fps: Fps::new(),
        }
    }

    pub fn init(&mut self, resources: Resources) {
        self.resources = resources;
        self.game_time.start();
    }

    pub fn run(&mut self) {
        let current_time = self.game_time.current_time();
        self.fps.calc(current_time);

        // Game paint iteration goes here

        self.fps.set(current_time);
    }

    pub fn handle_event(&self, name: GameEvent, _event: MouseEvent) {
        log!("Game handling event for: {}", name.to_string())
    }
}
