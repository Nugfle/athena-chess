use std::sync::LazyLock;

use game::Game;
use log::info;

use crate::engine::attack_tables::AttackTables;

#[cfg(feature = "benchmark")]
pub fn create_tables() { AttackTables::create_tables(); }

mod attack_tables;
mod board;
mod game;

/// safely holds on to our Attack Tables across multiple instances on Engine and multiple games
static ATTACK_TABLES: LazyLock<AttackTables> = LazyLock::new(|| {
    info!("building attack tables...");
    let start = std::time::Instant::now();
    let at = AttackTables::create_tables();
    let took = start.elapsed().as_millis();
    info!("finished building tables, took {} ms...", took);
    at
});

pub struct Engine {
    game: Option<Game>,
}
impl Engine {
    pub fn new() -> Self {
        println!("building engine");
        let table = &*ATTACK_TABLES;
        Self { game: None }
    }
    pub fn attach(&mut self, game: Game) {}
}
