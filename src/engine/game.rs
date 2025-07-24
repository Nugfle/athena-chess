use log::info;
use std::sync::LazyLock;

use attack_tables::AttackTables;
use board::BitBoard;
use board::piece::Piece;
use chess_move::Move;
use error::ChessError;

mod attack_tables;
mod board;
pub mod chess_move;
mod error;
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

#[derive(Debug, Clone)]
pub struct Game {
    board: BitBoard,
    moves: Vec<Move>,
}

impl Game {
    pub fn init() -> Self {
        let _ = ATTACK_TABLES;
        Self {
            board: BitBoard::init(),
            moves: Vec::new(),
        }
    }

    pub fn execute_move(&mut self, mv: Move) -> Result<(), ChessError> {
        match mv.get_piece() {
            Piece::Pawn => {
                todo!("compute pawn moves on the fly");
            }
            _ => todo!("implement move checks with ATTACK_TABLE"),
        }
        self.moves.push(mv);
        Ok(())
    }
}
