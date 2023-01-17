use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, MouseEvent};

use crate::constants::{CANVAS_HEIGHT, CANVAS_HEIGHT_F64, CANVAS_WIDTH, CANVAS_WIDTH_F64};
use crate::fps::Fps;
use crate::log;
use crate::model::{GameEvent, GameState};
use crate::painter::Painter;
use crate::resource_loader::Resources;
use crate::scene::HomeScene;
use crate::sprite::Sprite;
use crate::timers::GameTime;
use crate::web_utils::{create_canvas, get_canvas_context};

pub struct Game {
    pub resources: Resources,
    pub painter: Painter,

    sprites: Vec<Sprite>,
    state: GameState,
    game_time: GameTime,
    fps: Fps,
}

impl Game {
    pub fn new() -> Game {
        let canvas = create_canvas(CANVAS_WIDTH, CANVAS_HEIGHT);
        let context = get_canvas_context(&canvas);

        Game {
            painter: Painter::new(),
            resources: Resources::new(),
            game_time: GameTime::new(),
            state: GameState::new(),
            fps: Fps::new(),
            sprites: vec![],
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

        self.draw();

        // TODO - Handle Game internal sprites garbage collection Game::gc()

        // TODO - SunGenerator::tick()

        log!("Running game tick with {} Sprites", self.sprites.len());
        self.fps.set(current_time);
    }

    fn draw(&mut self) {
        self.painter.clear();

        self.sprites.iter().for_each(Painter::draw_sprite);
    }

    // Events //

    pub fn handle_event(&self, name: GameEvent, _event: MouseEvent) {
        log!("Game handling event for: {}", name.to_string())
    }

    // Game Actions //

    fn start_home_scene(&mut self) {
        self.reset_state();
        HomeScene::start(self);
    }

    // Game State mutations //

    pub fn reset_state(&mut self) {
        self.state = GameState::new();
        // TODO - Clear all sprites once available
    }

    pub fn add_sprites(&mut self, sprites: &mut Vec<Sprite>) {
        self.sprites.append(sprites);

        self.sprites.sort_by(|a, b| a.order.cmp(&b.order));
    }

    // Getters //

    pub fn canvas(&self) -> &HtmlCanvasElement {
        &self.painter.canvas
    }
}
