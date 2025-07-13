use std::usize;

use crate::engine::chess_move::Move;
use crate::engine::piece::{Color, Piece};
use crate::engine::square::Square;
use thiserror::Error;

#[derive(Debug, Clone, Copy, Error)]
#[error("Illegal Move {piece} from {from} to {to}")]
pub struct IllegalMoveError {
    from: Square,
    to: Square,
    piece: Piece,
}

impl From<Move> for IllegalMoveError {
    fn from(value: Move) -> Self {
        Self {
            from: value.get_from(),
            to: value.get_to(),
            piece: value.get_piece(),
        }
    }
}

pub struct Board {
    board: [[Option<Piece>; 8]; 8],
}

impl Board {
    pub fn square_is_occupied(&self, square: Square) -> bool {
        self.get_piece_on_square(square).is_some()
    }

    pub fn get_piece_on_square(&self, square: Square) -> Option<&Piece> {
        self.board[square.vertical() as usize][square.horizontal() as usize].as_ref()
    }

    pub fn put_piece(&mut self, square: Square, piece: Piece) {
        self.board[square.vertical() as usize][square.horizontal() as usize] = Some(piece)
    }

    pub fn clear_square(&mut self, square: Square) {
        self.board[square.vertical() as usize][square.horizontal() as usize] = None
    }

    pub fn move_piece(&mut self, from: Square, to: Square) {
        self.board[to.horizontal() as usize][from.vertical() as usize] =
            std::mem::take(&mut self.board[from.horizontal() as usize][from.vertical() as usize]);
    }
}
