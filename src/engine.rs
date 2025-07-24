use game::Game;
use log::info;

pub mod game;

pub struct Engine {
    games: Vec<Game>,
}

impl Engine {
    pub fn new() -> Self {
        info!("creating engine...");
        Self { games: Vec::new() }
    }
}
