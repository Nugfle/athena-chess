use std::sync::LazyLock;

use game::Game;
use log::info;

use crate::engine::attack_tables::AttackTables;
use crate::engine::game::Move;

// used for benchmarking as AttackTables is private and usually can't be accessed
#[cfg(feature = "benchmark")]
pub fn create_tables() { AttackTables::create_tables(); }

mod attack_tables;
mod board;
mod evaluation;
mod game;

/// safely holds on to our Attack Tables across multiple instances on Engine and multiple games
static ATTACK_TABLES: LazyLock<AttackTables> = LazyLock::new(|| {
    let start = std::time::Instant::now();
    let at = AttackTables::create_tables();
    let took = start.elapsed().as_millis();
    info!("built attack tables, took {} ms...", took);
    at
});

/// The core of the chess engine, that holds on to games and the attached analyzers
pub struct Engine {
    games: Vec<Game>,
}

impl Engine {
    pub fn new() -> Self {
        info!("creating engine...");
        let _ = &*ATTACK_TABLES;
        Self { games: Vec::new() }
    }
}
