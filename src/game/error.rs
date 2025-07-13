use std::convert::Infallible;

use super::{board::IllegalMoveError, square::InvalidSquareError};

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
