use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::Closure;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::spawn_local;
use web_sys::MouseEvent;

use crate::game::Game;
use crate::log;
use crate::model::GameEvent;
use crate::resource_loader::ResourceLoader;
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

#[derive(Debug)]
pub enum EngineError {
    IO(std::io::Error),
    Js(JsValue),
    SerdeParsing(serde_wasm_bindgen::Error),
}

impl From<JsValue> for EngineError {
    fn from(e: JsValue) -> Self {
        EngineError::Js(e)
    }
}

impl From<serde_wasm_bindgen::Error> for EngineError {
    fn from(e: serde_wasm_bindgen::Error) -> Self {
        EngineError::SerdeParsing(e)
    }
}

impl From<EngineError> for JsValue {
    fn from(e: EngineError) -> Self {
        match e {
            EngineError::Js(e) => e,
            EngineError::SerdeParsing(e) => JsValue::from_str(&e.to_string()),
            EngineError::IO(e) => JsValue::from_str(&e.to_string()),
        }
    }
}

impl Engine {
    pub fn launch() {
        spawn_local(async move {
            let engine = Engine::default();

            // Load assets
            engine.game.borrow_mut().resources = ResourceLoader::load(&ResourceLoader).await;

            // Attach game listeners
            engine.register_events();

            // Starts main loop
            engine.start_game_loop();
        })
    }

    fn register_events(&self) {
        self.handled_events
            .iter()
            .for_each(|event| self.listen_event(*event));
    }

    fn listen_event(&self, name: GameEvent) {
        let game_closure_ref = Rc::clone(&self.game);

        let closure = Closure::wrap(Box::new(move |event: MouseEvent| {
            game_closure_ref.borrow_mut().handle_event(name, event);
        }) as Box<dyn FnMut(_)>);

        self.game
            .borrow()
            .canvas
            .add_event_listener_with_callback(
                &name.to_string().to_lowercase(),
                closure.as_ref().unchecked_ref(),
            )
            .unwrap();

        closure.forget();
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
