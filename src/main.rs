#![recursion_limit = "256"]
use athena_chess::game::Game;

mod game;
fn main() {
    env_logger::init();

    let game = Game::new();
    println!("{}", game);
}
