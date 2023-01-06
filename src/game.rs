use crate::log;

#[derive(Debug)]
pub struct Game;

impl Game {
    pub fn new() -> Game {
        Game {}
    }

    pub fn run(&mut self) {
        log!("Game Run iteration");
    }
}
