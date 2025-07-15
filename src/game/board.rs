use super::piece::{Color, Piece};
use super::square::Square;
use colored::Colorize;
use std::fmt::Display;
use thiserror::Error;

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

/** The Default Board implementation
# Example
```
let mut board = Board::init();
board.make_move("e2".parse().unwrap(),, "e4".parse().unwrap());

assert!(board.get_piece_on_square("e2").is_none());
assert_eq!(board.get_piece_on_square("e4"), Some(Piece::Pawn(Color::White)));
```
*/
#[derive(Debug, Clone)]
pub struct ArrayBoard {
    board: [[Option<Piece>; 8]; 8],
}

impl Display for ArrayBoard {
    /// a pretty printed chess board with the Capital Letters as Pieces
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

pub trait Board {
    // template methods
    fn get_piece_on_square(&self, square: Square) -> Option<Piece>;
    fn put_piece_option(&mut self, square: Square, piece: Option<Piece>);
    fn clear_square(&mut self, square: Square);

    fn put_piece(&mut self, square: Square, piece: Piece) {
        self.put_piece_option(square, Some(piece));
    }

    fn init() -> Self
    where
        Self: Default,
    {
        let mut board = Self::default();
        board.put_piece("e1".parse().unwrap(), Piece::King(Color::White));
        board.put_piece("d1".parse().unwrap(), Piece::Queen(Color::White));
        board.put_piece("c1".parse().unwrap(), Piece::Bishop(Color::White));
        board.put_piece("f1".parse().unwrap(), Piece::Bishop(Color::White));
        board.put_piece("b1".parse().unwrap(), Piece::Knight(Color::White));
        board.put_piece("g1".parse().unwrap(), Piece::Knight(Color::White));
        board.put_piece("h1".parse().unwrap(), Piece::Rook(Color::White));
        board.put_piece("a1".parse().unwrap(), Piece::Rook(Color::White));

        board.put_piece("e8".parse().unwrap(), Piece::King(Color::Black));
        board.put_piece("d8".parse().unwrap(), Piece::Queen(Color::Black));
        board.put_piece("c8".parse().unwrap(), Piece::Bishop(Color::Black));
        board.put_piece("f8".parse().unwrap(), Piece::Bishop(Color::Black));
        board.put_piece("b8".parse().unwrap(), Piece::Knight(Color::Black));
        board.put_piece("g8".parse().unwrap(), Piece::Knight(Color::Black));
        board.put_piece("h8".parse().unwrap(), Piece::Rook(Color::Black));
        board.put_piece("a8".parse().unwrap(), Piece::Rook(Color::Black));

        board.put_piece("a7".parse().unwrap(), Piece::Pawn(Color::Black));
        board.put_piece("b7".parse().unwrap(), Piece::Pawn(Color::Black));
        board.put_piece("c7".parse().unwrap(), Piece::Pawn(Color::Black));
        board.put_piece("d7".parse().unwrap(), Piece::Pawn(Color::Black));
        board.put_piece("e7".parse().unwrap(), Piece::Pawn(Color::Black));
        board.put_piece("f7".parse().unwrap(), Piece::Pawn(Color::Black));
        board.put_piece("g7".parse().unwrap(), Piece::Pawn(Color::Black));
        board.put_piece("h7".parse().unwrap(), Piece::Pawn(Color::Black));

        board.put_piece("a2".parse().unwrap(), Piece::Pawn(Color::White));
        board.put_piece("b2".parse().unwrap(), Piece::Pawn(Color::White));
        board.put_piece("c2".parse().unwrap(), Piece::Pawn(Color::White));
        board.put_piece("d2".parse().unwrap(), Piece::Pawn(Color::White));
        board.put_piece("e2".parse().unwrap(), Piece::Pawn(Color::White));
        board.put_piece("f2".parse().unwrap(), Piece::Pawn(Color::White));
        board.put_piece("g2".parse().unwrap(), Piece::Pawn(Color::White));
        board.put_piece("h2".parse().unwrap(), Piece::Pawn(Color::White));

        board
    }

    /// Moves whatever is at *from* to *to*.
    /// Note that this method doesn't care, whether the move is valid or not.
    fn make_move(&mut self, from: Square, to: Square) {
        let p = self.get_piece_on_square(from);
        self.put_piece_option(to, p);
        self.clear_square(from);
    }
}

impl Default for ArrayBoard {
    /// an empty board, use `init()` to set up all the pieces
    fn default() -> Self {
        Self { board: [[None; 8]; 8] }
    }
}

impl Board for ArrayBoard {
    fn get_piece_on_square(&self, square: Square) -> Option<Piece> {
        self.board[square.vertical() as usize][square.horizontal() as usize]
    }

    /// overwrites the previous value at *square* with *piece*
    fn put_piece_option(&mut self, square: Square, piece: Option<Piece>) {
        self.board[square.vertical() as usize][square.horizontal() as usize] = piece
    }

    /// sets the value at *square* to `None`
    fn clear_square(&mut self, square: Square) {
        self.board[square.vertical() as usize][square.horizontal() as usize] = None
    }
}

#[cfg(test)]
mod test {}
