use web_sys::{HtmlCanvasElement, MouseEvent};

use crate::fps::Fps;
use crate::log;
use crate::model::{GameEvent, GameState, Position};
use crate::painter::Painter;
use crate::resource_loader::Resources;
use crate::scene::HomeScene;
use crate::sprite::{BehaviorManager, Sprite, SpriteMutation};
use crate::timers::GameTime;

pub struct Game {
    pub resources: Resources,
    pub painter: Painter,
    pub game_time: GameTime,

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

        let mutations = self
            .sprites
            .iter_mut()
            .map(|sprite| {
                BehaviorManager::run_sprite_behaviours(
                    sprite,
                    self.game_time.time,
                    self.game_time.last_timestamp,
                    &Position {
                        left: 0.0,
                        top: 0.0,
                    },
                    &self.painter.context,
                )
            })
            .flatten()
            .collect();

        // Handle add mutations
        self.handle_sprite_mutation(mutations);

        // Draw update Sprites
        self.sprites
            .iter()
            .for_each(|sprite| self.painter.draw_sprite(sprite));
    }

    // Events //

    pub fn handle_event(&self, name: GameEvent, _event: MouseEvent) {
        log!("Game handling event for: {}", name.to_string())
    }

    pub fn handle_sprite_mutation(&mut self, mutations: Vec<SpriteMutation>) {
        log!("Handle sprite mutation: {}", mutations.len())
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
