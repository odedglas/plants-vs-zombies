use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::Closure;

use crate::game::Game;
use crate::log;
use crate::model::GameEvent;
use crate::web_utils::request_animation_frame;

pub struct Engine {
    game: Rc<RefCell<Game>>,
    handled_events: Vec<GameEvent>,
}

impl Default for Engine {
    fn default() -> Self {
        Engine {
            game: Rc::new(RefCell::new(Game::new())),
            handled_events: vec![
                GameEvent::Mousemove,
                GameEvent::Mouseup,
                GameEvent::Mouseleave,
                GameEvent::Mouseenter,
                GameEvent::Mousedown,
            ],
        }
    }
}

impl Engine {
    pub fn launch() {
        // Load assets

        // Create game

        // Attach listeners
        let engine = Engine::default();

        log!("Engine launched {}", engine.handled_events.len());

        engine.start_game_loop();
    }

    fn start_game_loop(&self) {
        // This reference will point to the closure that will recursively called in each animation frame trigger.
        // Thus this is a persistence RC which is used in all future iterations.
        let main_loop_ref = Rc::new(RefCell::new(None));

        //This reference will hit the initial animation frame and be dropped by the end of this scope.
        let initial_trigger_ref = Rc::clone(&main_loop_ref);

        let game = Rc::clone(&self.game);
        let mut iter = 0;
        *initial_trigger_ref.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            let mut game = game.borrow_mut();
            iter += 1;

            game.run();

            if iter > 150 {
                log!("Game done");
                let _ = main_loop_ref.borrow_mut().take();
                return;
            }

            request_animation_frame(main_loop_ref.borrow().as_ref().unwrap());
        }) as Box<dyn FnMut()>));

        request_animation_frame(initial_trigger_ref.borrow().as_ref().unwrap());
    }
}
