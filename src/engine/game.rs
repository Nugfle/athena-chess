use log::info;
use std::sync::LazyLock;

use attack_tables::AttackTables;
use board::BitBoard;
use board::piece::Piece;
use board::square::Square;

mod attack_tables;
mod board;
mod evaluation;

static ATTACK_TABLES: LazyLock<AttackTables> = LazyLock::new(|| {
    let start = std::time::Instant::now();
    let at = AttackTables::create_tables();
    let took = start.elapsed().as_millis();
    info!("built attack tables, took {} ms...", took);
    at
});

#[cfg(feature = "benchmark")]
pub fn create_tables() { AttackTables::create_tables(); }

#[derive(Debug, Clone, Copy)]
pub struct Move {
    piece: Piece,
    from: Square,
    to: Square,
}

#[derive(Debug, Clone)]
pub struct Game {
    board: BitBoard,
    moves: Vec<Move>,
}

impl Game {
    pub fn init() -> Self {
        Self {
            board: BitBoard::init(),
            moves: Vec::new(),
        }
    }
}
