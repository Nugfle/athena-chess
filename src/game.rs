use std::fmt::Display;

use board::Board;
use chess_move::Move;
use piece::Color;

mod board;
mod chess_move;
mod error;
mod piece;
mod square;

#[derive(Debug, Clone)]
pub struct Game {
    moves: Vec<Move>,
    turn: Color,
    board: Board,
}
impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.board.to_string())
    }
}

impl Game {
    pub fn new() -> Self {
        return Self {
            moves: Vec::new(),
            turn: Color::White,
            board: Board::init(),
        };
    }
}
