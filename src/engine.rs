use std::sync::LazyLock;

use game::Game;
use log::info;

use crate::engine::attack_tables::AttackTables;

// used for benchmarking as AttackTables is private and usually can't be accessed
#[cfg(feature = "benchmark")]
pub fn create_tables() { AttackTables::create_tables(); }

mod attack_tables;
mod board;
mod game;

/// safely holds on to our Attack Tables across multiple instances on Engine and multiple games
static ATTACK_TABLES: LazyLock<AttackTables> = LazyLock::new(|| {
    let start = std::time::Instant::now();
    let at = AttackTables::create_tables();
    let took = start.elapsed().as_millis();
    info!("built attack tables, took {} ms...", took);
    at
});

/// The Core Component which holds onto a game, evaluates the Position and suggests the next best
/// move.
pub struct Engine {
    game: Option<Game>,
}
impl Engine {
    pub fn new() -> Self {
        info!("creating engine...");
        let table = &*ATTACK_TABLES;
        Self { game: None }
    }
    pub fn attach(&mut self, game: Game) {}
}
