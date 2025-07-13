use std::convert::Infallible;

use crate::engine::{board::IllegalMoveError, square::InvalidSquareError};

use thiserror::Error;

#[derive(Debug, Clone, Copy, Error)]
pub enum ChessError {
    #[error(transparent)]
    IllegalMoveError(#[from] IllegalMoveError),
    #[error(transparent)]
    InvalidSquareError(#[from] InvalidSquareError),
    #[error(transparent)]
    Infallible(#[from] Infallible),
}
