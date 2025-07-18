use game::ArrayBoard;
use game::Game;

mod game;

pub struct Engine {
    game: Game<ArrayBoard>,
}

impl Engine {
    pub fn new() -> Self {
        Self { game: Game::new() }
    }
    pub fn print_game(&self) -> String {
        self.game.to_string()
    }
}
