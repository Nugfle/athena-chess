use std::convert::Infallible;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum ChessError {
    #[error(transparent)]
    IllegalMoveError(#[from] IllegalMoveError),
    #[error(transparent)]
    InvalidSquareError(#[from] InvalidSquareError),
    #[error(transparent)]
    Infallible(#[from] Infallible),
}

#[derive(Debug, Clone, Copy, Error)]
pub enum IllegalMoveError {
    #[error("Illegal Move: There is no piece on the 'from' Square")]
    NoPieceOnSquare,
    #[error("Illegal Move: Can't take piece of the same color")]
    TakesSameColor,
    #[error("Illegal Move: The Move violates movement rules for the piece")]
    InvalidMoveForPiece,
    #[error("Illegal Move: The Move is for a piece of the opponents color")]
    NotYourPiece,
    #[error("Illegal Move: The Move is blocked by another piece")]
    Blocked,
    #[error("Illegal Move: The Piece is pinned in place against the King")]
    Pinned,
}

#[derive(Error, Debug, Clone)]
pub enum InvalidSquareError {
    #[error("Can't create Square, out of bounds: ({h}, {v})")]
    OutOfBounds { h: u8, v: u8 },
    #[error("The Literal: '{literal}' is not of lenght 2: {length}")]
    InvalidLiteralLength { literal: String, length: usize },
    #[error("The Literal: '{literal}' is not valid chess notation for a square")]
    InvalidLiteral { literal: String },
}
