use std::cell::RefCell;
use std::rc::Rc;

use crate::game::Game;
use crate::log;
use crate::model::GameEvent;

pub struct Engine {
    game: Rc<RefCell<Game>>,
    handled_events: Vec<GameEvent>,
}

impl Engine {
    pub fn launch() {
        // Load assets

        // Create game

        // Attach listeners

        log!("Engine launched");
    }
}
