use std::fmt::Display;

use board::Board;
use chess_move::Move;
use piece::Color;

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
    turn: Color,
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
            turn: Color::White,
            board: ArrayBoard::init(),
        };
    }
}

impl<T> Game<T>
where
    T: Sized + Board,
{
    pub fn get_legal_moves(&self) -> Vec<Move> {
        let pieces = match self.turn {
            Color::Black => self.board.get_black_pieces(),
            Color::White => self.board.get_white_pieces(),
        };
        let mut moves = Vec::new();
        todo!("implement the logic go get all legal moves");
        moves
    }

    fn get_legal_moves_pawn(&self, square: Square, color: &Color) -> Vec<Move> {
        let mut moves = Vec::new();

        return moves;
    }
    fn get_legal_moves_knight(&self, square: Square, color: &Color) -> Vec<Move> {
        let mut moves = Vec::new();
        return moves;
    }
    fn get_legal_moves_bishop(&self, square: Square, color: &Color) -> Vec<Move> {
        let mut moves = Vec::new();
        return moves;
    }
    fn get_legal_moves_rook(&self, square: Square, color: &Color) -> Vec<Move> {
        let mut moves = Vec::new();
        return moves;
    }
    fn get_legal_moves_queen(&self, square: Square, color: &Color) -> Vec<Move> {
        let mut moves = Vec::new();
        return moves;
    }
    fn get_legal_moves_king(&self, square: Square, color: &Color) -> Vec<Move> {
        let mut moves = Vec::new();
        return moves;
    }
}
