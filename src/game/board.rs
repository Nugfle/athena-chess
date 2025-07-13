use std::fmt::Display;

use super::chess_move::Move;
use super::piece::{Color, Piece};
use super::square::Square;
use colored::Colorize;
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

#[derive(Debug, Clone)]
pub struct Board {
    board: [[Option<Piece>; 8]; 8],
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for v in 0..8 {
            for h in 0..8 {
                write!(
                    f,
                    "{}",
                    if (v + h) % 2 == 0 {
                        match self.get_piece_on_square((h, v).try_into().unwrap()) {
                            None => "   ".normal(),
                            Some(p) => match p.get_color() {
                                Color::Black => format!(" {} ", p.short_name()).black(),
                                Color::White => format!(" {} ", p.short_name()).bright_white(),
                            },
                        }
                        .on_bright_black()
                    } else {
                        match self.get_piece_on_square((h, v).try_into().unwrap()) {
                            None => "   ".normal(),
                            Some(p) => match p.get_color() {
                                Color::Black => format!(" {} ", p.short_name()).black(),
                                Color::White => format!(" {} ", p.short_name()).bright_white(),
                            },
                        }
                        .on_white()
                    }
                )?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Default for Board {
    /// sets the board for a new game of chess
    fn default() -> Self {
        Self::init()
    }
}

impl Board {
    pub fn new() -> Self {
        Self { board: [[None; 8]; 8] }
    }
    pub fn init() -> Self {
        let mut board = Board::new();
        board.put_piece("e1".try_into().unwrap(), Piece::King(Color::White));
        board.put_piece("d1".try_into().unwrap(), Piece::Queen(Color::White));
        board.put_piece("c1".try_into().unwrap(), Piece::Bishop(Color::White));
        board.put_piece("f1".try_into().unwrap(), Piece::Bishop(Color::White));
        board.put_piece("b1".try_into().unwrap(), Piece::Knight(Color::White));
        board.put_piece("g1".try_into().unwrap(), Piece::Knight(Color::White));
        board.put_piece("h1".try_into().unwrap(), Piece::Rook(Color::White));
        board.put_piece("a1".try_into().unwrap(), Piece::Rook(Color::White));

        board.put_piece("e8".try_into().unwrap(), Piece::King(Color::Black));
        board.put_piece("d8".try_into().unwrap(), Piece::Queen(Color::Black));
        board.put_piece("c8".try_into().unwrap(), Piece::Bishop(Color::Black));
        board.put_piece("f8".try_into().unwrap(), Piece::Bishop(Color::Black));
        board.put_piece("b8".try_into().unwrap(), Piece::Knight(Color::Black));
        board.put_piece("g8".try_into().unwrap(), Piece::Knight(Color::Black));
        board.put_piece("h8".try_into().unwrap(), Piece::Rook(Color::Black));
        board.put_piece("a8".try_into().unwrap(), Piece::Rook(Color::Black));

        board.put_piece("a7".try_into().unwrap(), Piece::Pawn(Color::Black));
        board.put_piece("b7".try_into().unwrap(), Piece::Pawn(Color::Black));
        board.put_piece("c7".try_into().unwrap(), Piece::Pawn(Color::Black));
        board.put_piece("d7".try_into().unwrap(), Piece::Pawn(Color::Black));
        board.put_piece("e7".try_into().unwrap(), Piece::Pawn(Color::Black));
        board.put_piece("f7".try_into().unwrap(), Piece::Pawn(Color::Black));
        board.put_piece("g7".try_into().unwrap(), Piece::Pawn(Color::Black));
        board.put_piece("h7".try_into().unwrap(), Piece::Pawn(Color::Black));

        board.put_piece("a2".try_into().unwrap(), Piece::Pawn(Color::White));
        board.put_piece("b2".try_into().unwrap(), Piece::Pawn(Color::White));
        board.put_piece("c2".try_into().unwrap(), Piece::Pawn(Color::White));
        board.put_piece("d2".try_into().unwrap(), Piece::Pawn(Color::White));
        board.put_piece("e2".try_into().unwrap(), Piece::Pawn(Color::White));
        board.put_piece("f2".try_into().unwrap(), Piece::Pawn(Color::White));
        board.put_piece("g2".try_into().unwrap(), Piece::Pawn(Color::White));
        board.put_piece("h2".try_into().unwrap(), Piece::Pawn(Color::White));

        board
    }
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
        self.board[to.vertical() as usize][from.horizontal() as usize] =
            std::mem::take(&mut self.board[from.vertical() as usize][from.horizontal() as usize]);
    }
}
