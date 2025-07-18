use std::fmt::Display;

use board::Board;
use chess_move::Move;

use crate::game::{array_board::ArrayBoard, square::Square};

mod array_board;
mod board;
mod chess_move;
mod error;
mod piece;
mod square;

/// the representation of a games state, containing all the moves made together with the active color
/// and current state of the Board.
#[derive(Debug, Clone)]
pub struct Game<T>
where
    T: Sized + Board,
{
    moves: Vec<Move>,
    board: T,
}

impl<T> Display for Game<T>
where
    T: Sized + Board + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.board.to_string())
    }
}

impl Game<ArrayBoard> {
    pub fn new() -> Self {
        return Self {
            moves: Vec::new(),
            board: ArrayBoard::init(),
        };
    }
}

impl<T> Game<T>
where
    T: Sized + Board,
{
    pub fn get_legal_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();
        todo!("implement the logic go get all legal moves");
        moves
    }
}
