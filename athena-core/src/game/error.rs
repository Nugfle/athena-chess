use thiserror::Error;

use super::board::piece::{Color, Piece};
use super::board::square::Square;
use super::chess_move::Move;

#[derive(Debug, Clone, Copy, Error)]
pub enum ChessError {
    #[error("Invalid Square: {square}")]
    InvalidSquare { square: u8 },

    #[error("Illegal Move: {e}")]
    IllegalMove { e: IllegalMoveError },
}

#[derive(Debug, Clone, Copy, Error, PartialEq, Eq)]
pub enum IllegalMoveError {
    #[error("Empty Square: {square}.")]
    EmptySquare { square: Square },

    #[error("Your King is in Check.")]
    IsInCheck,

    #[error("not a valid move for piece: {mv}.")]
    MoveInvalid { mv: Move },

    #[error("can't move piece on square: {square}. Not your color: {color}.")]
    NotYourPiece { color: Color, square: Square },

    #[error("the piece on the square an the piece in the move don't match. Expected: {expected}, Found: {found}.")]
    DifferentPiece { expected: Piece, found: Piece },

    #[error("the move: {mv}, takes your own piece: {piece}.")]
    TakesOwnPiece { mv: Move, piece: Piece },

    #[error("the move: {mv}, tries to take an empty square: {square} with a pawn")]
    TakesEmptySquare { mv: Move, square: Square },

    #[error("cant do the move: {mv}, the square: {square} is blocked")]
    Blocked { mv: Move, square: Square },
}
