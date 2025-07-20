use game::Game;

mod game;

pub struct Engine {
    game: Game,
}
impl Engine {
    pub fn new() -> Self {
        Self { game: Game {} }
    }
}
