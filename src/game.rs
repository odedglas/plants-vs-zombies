use web_sys::{HtmlCanvasElement, MouseEvent};

use crate::fps::Fps;
use crate::log;
use crate::model::{BehaviorType, GameEvent, GameState, Position};
use crate::painter::Painter;
use crate::resource_loader::Resources;
use crate::scene::HomeScene;
use crate::sprite::{BehaviorManager, Sprite, SpriteMutation};
use crate::timers::GameTime;

pub struct Game {
    pub resources: Resources,
    pub painter: Painter,
    pub game_time: GameTime,
    pub mouse_position: Position,

    sprites: Vec<Sprite>,
    state: GameState,
    fps: Fps,
}

impl Game {
    pub fn new() -> Game {
        Game {
            painter: Painter::new(),
            resources: Resources::new(),
            game_time: GameTime::new(),
            state: GameState::new(),
            fps: Fps::new(),
            mouse_position: Position::new(0.0, 0.0),
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

        self.fps.set(current_time);
    }

    fn draw(&mut self) {
        self.painter.clear();

        self.sprites.iter_mut().for_each(|sprite| {
            // Collect mutations
            let mutations = BehaviorManager::run(
                sprite,
                &self.game_time,
                &self.mouse_position,
                &self.painter.context,
            );

            // Apply on Sprite
            sprite.apply_mutation(mutations);

            self.painter.draw_sprite(sprite);
        });
    }

    // Events //

    pub fn handle_event(&mut self, event_name: GameEvent, event: MouseEvent) {
        log!("Game handling event for: {}", event_name.to_string());
        let current_mouse = Position::from_event(event);
        self.mouse_position = current_mouse;

        match event_name {
            GameEvent::MouseMove => self.toggle_game_behavior(true, &[BehaviorType::Hover]),
            GameEvent::MouseDown => self.toggle_game_behavior(true, &[BehaviorType::Click]),
            GameEvent::MouseUp => self.toggle_game_behavior(false, &[BehaviorType::Click]),
            GameEvent::MouseLeave => self.toggle_game_behavior(false, &[BehaviorType::Hover]),
        }
    }

    pub fn toggle_game_behavior(&mut self, active: bool, types: &[BehaviorType]) {
        BehaviorManager::toggle_behaviors(self.sprites.iter(), types, active, self.game_time.time)
    }

    // Game Actions //
    pub fn game_over(&mut self) {
        self.painter.clear();
    }

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
