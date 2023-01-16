use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, MouseEvent};

use crate::fps::Fps;
use crate::log;
use crate::model::{GameEvent, GameState};
use crate::resource_loader::{ResourceKind, Resources};
use crate::timers::GameTime;
use crate::web_utils::{create_canvas, get_canvas_context};

pub struct Game {
    pub resources: Resources,
    pub canvas: HtmlCanvasElement,
    pub context: CanvasRenderingContext2d,

    state: GameState,
    game_time: GameTime,
    fps: Fps,
}

impl Game {
    pub fn new() -> Game {
        let canvas = create_canvas(600, 400);
        let context = get_canvas_context(&canvas);

        Game {
            canvas,
            context,
            resources: Resources::new(),
            game_time: GameTime::new(),
            state: GameState::new(),
            fps: Fps::new(),
        }
    }

    pub fn init(&mut self, resources: Resources) {
        self.resources = resources;

        // Start game internal time
        self.game_time.start();

        // Paint home scene
        self.start_home_scene();
    }

    pub fn run(&mut self) {
        let current_time = self.game_time.current_time();
        self.fps.calc(current_time);

        // Game paint iteration goes here

        self.fps.set(current_time);
    }

    // Events
    pub fn handle_event(&self, name: GameEvent, _event: MouseEvent) {
        log!("Game handling event for: {}", name.to_string())
    }

    // Game Actions
    fn start_home_scene(&mut self) {
        let resource = self.resources.get_resource(
            "SelectorAdventureShadow",
            ResourceKind::Interface
        );

        log!("Resource Cell, {:?}", resource.cell);
        log!("Resource data, {:?}", resource.data);

        // TODO - Clear all sprites once available
    }
}
