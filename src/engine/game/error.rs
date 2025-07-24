use thiserror::Error;

use super::board::square::Square;
use super::chess_move::Move;

#[derive(Debug, Clone, Copy, Error)]
pub enum ChessError {
    #[error("Invalid Square: {square}")]
    InvalidSquare { square: u8 },

    #[error("Illegal Move: {e}")]
    IllegalMove { e: IllegalMoveError },
}

#[derive(Debug, Clone, Copy, Error)]
pub enum IllegalMoveError {
    #[error("Empty Square")]
    EmptySquare { s: Square },

    #[error("Is in Check")]
    IsInCheck,

    #[error("not a valid move for piece: {m}")]
    MoveInvalid { m: Move },
}
