use std::fmt::Display;

use board::Board;
use chess_move::Move;
use piece::Color;

use crate::game::{piece::Piece, square::Square};

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

    pub fn get_legal_moves(&self) -> Vec<Move> {
        let pieces = self.board.get_pieces_of_color(self.turn);
        let mut moves = Vec::new();
        for (piece, square) in pieces {
            let mut m = match piece {
                Piece::Pawn(_) => self.get_legal_moves_pawn(square),
                Piece::Knight(_) => self.get_legal_moves_knight(square),
                Piece::Bishop(_) => self.get_legal_moves_bishop(square),
                Piece::Rook(_) => self.get_legal_moves_rook(square),
                Piece::Queen(_) => self.get_legal_moves_queen(square),
                Piece::King(_) => self.get_legal_moves_king(square),
            };
            moves.append(&mut m);
        }
        moves
    }
}

impl Game {
    fn get_legal_moves_pawn(&self, square: Square) -> Vec<Move> {
        let mut moves = Vec::new();
        return moves;
    }
    fn get_legal_moves_knight(&self, square: Square) -> Vec<Move> {
        let mut moves = Vec::new();
        return moves;
    }
    fn get_legal_moves_bishop(&self, square: Square) -> Vec<Move> {
        let mut moves = Vec::new();
        return moves;
    }
    fn get_legal_moves_rook(&self, square: Square) -> Vec<Move> {
        let mut moves = Vec::new();
        return moves;
    }
    fn get_legal_moves_queen(&self, square: Square) -> Vec<Move> {
        let mut moves = Vec::new();
        return moves;
    }
    fn get_legal_moves_king(&self, square: Square) -> Vec<Move> {
        let mut moves = Vec::new();
        return moves;
    }
}
