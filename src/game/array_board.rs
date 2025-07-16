use crate::game::board::Board;
use crate::game::piece::{Color, Piece};
use crate::game::square::Square;
use colored::Colorize;
use core::fmt::Display;

#[derive(Debug, Clone)]
pub struct ArrayBoard {
    /// # Order
    /// 0 - King 
    /// 1 - Queen
    /// 2,3 - a,h Rook
    /// 4,5 - c,f Bishop
    /// 6,7 - b,g Knight
    /// 8..15 - a..h Pawn
    white_pieces: [Square; 16],
    white_flags: u16,
    /// # Order
    /// 0 - King 
    /// 1 - Queen
    /// 2,3 - a,h Rook
    /// 4,5 - c,f Bishop
    /// 6,7 - b,g Knight
    /// 8..15 - a..h Pawn
    black_pieces: [Square; 16],
    black_flags: u16,
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

impl Default for ArrayBoard {
    /// an empty board, use `init()` to set up all the pieces
    fn default() -> Self {
        Self {
            board: [[None; 8]; 8],
            white_pieces: [None; 16],
            black_pieces: [None; 16],
        }
    }
}

impl Board for ArrayBoard {
    fn get_piece_on_square(&self, square: Square) -> Option<Piece> {
        self.board[square.vertical() as usize][square.horizontal() as usize]
    }

    /// overwrites the previous value at *square* with *piece*
    fn put_piece_option(&mut self, square: Square, piece: Option<Piece>) {
        if 
        self.board[square.vertical() as usize][square.horizontal() as usize] = piece
    }

    /// sets the value at *square* to `None`
    fn clear_square(&mut self, square: Square) {
        self.board[square.vertical() as usize][square.horizontal() as usize] = None
    }

    fn get_white_pieces(&self) -> [Option<(Piece, Square)>; 16] {
        self.white_pieces
    }
    fn get_black_pieces(&self) -> [Option<(Piece, Square)>; 16] {
        self.black_pieces
    }
}
