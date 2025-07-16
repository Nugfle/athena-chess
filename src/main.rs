#![recursion_limit = "256"]
use athena_chess::game::Game;

mod game;
fn main() {
    let game = Game::new();
    println!("{}", game);
    env_logger::init();
}
