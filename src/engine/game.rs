use std::fmt::Display;

pub use array_board::ArrayBoard;
use board::Board;
use chess_move::Move;

use crate::engine::game::{piece::Piece, square::Square};

mod array_board;
mod board;
mod chess_move;
mod error;
mod piece;
mod square;

#[derive(Debug, Clone, Copy)]
enum Color {
    White,
    Black,
}
/// the representation of a games state, containing all the moves made together with the active color
/// and current state of the Board.
#[derive(Debug, Clone)]
pub struct Game<T>
where
    T: Sized + Board,
{
    turn: Color,
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
            turn: Color::White,
        };
    }
}

impl<T> Game<T>
where
    T: Sized + Board,
{
    pub fn get_legal_moves(&self) -> Vec<Move> {
        let mut legal_moves = Vec::new();

        let pieces = match self.turn {
            Color::White => self.board.get_white_pieces(),
            Color::Black => self.board.get_black_pieces(),
        };

        for opt in pieces {
            if let Some((sq, pc)) = opt {
                let mut moves = match pc {
                    Piece::WhitePawn => self.get_white_pawn_moves(sq),
                    Piece::BlackPawn => self.get_black_pawn_moves(sq),
                    Piece::WhiteKnight | Piece::BlackKnight => self.get_knight_moves(sq, self.turn),
                    Piece::WhiteBishop | Piece::BlackBishop => self.get_bishop_moves(sq, self.turn),
                    Piece::WhiteRook | Piece::BlackRook => self.get_rook_moves(sq, self.turn),
                    Piece::WhiteQueen | Piece::BlackQueen => self.get_queen_moves(sq, self.turn),
                    Piece::WhiteKing | Piece::BlackKing => self.get_king_moves(sq, self.turn),
                };
                legal_moves.append(&mut moves);
            }
            break;
        }
        legal_moves
    }
}

impl<T> Game<T>
where
    T: ?Sized + Board,
{
    fn get_white_pawn_moves(&self, square: Square) -> Vec<Move> {
        Vec::new()
    }
    fn get_black_pawn_moves(&self, square: Square) -> Vec<Move> {
        Vec::new()
    }
    fn get_knight_moves(&self, square: Square, color: Color) -> Vec<Move> {
        Vec::new()
    }
    fn get_bishop_moves(&self, square: Square, color: Color) -> Vec<Move> {
        Vec::new()
    }
    fn get_rook_moves(&self, square: Square, color: Color) -> Vec<Move> {
        Vec::new()
    }
    fn get_queen_moves(&self, square: Square, color: Color) -> Vec<Move> {
        Vec::new()
    }
    fn get_king_moves(&self, square: Square, color: Color) -> Vec<Move> {
        Vec::new()
    }

    pub fn white_is_check(&self) -> bool {
        todo!()
    }
    pub fn black_is_check(&self) -> bool {
        todo!()
    }
}
