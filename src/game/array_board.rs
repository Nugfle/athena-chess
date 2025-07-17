use crate::game::board::Board;
use crate::game::piece::Piece;
use crate::game::square::*;
use colored::Colorize;
use core::fmt::Display;

#[derive(Debug, Clone)]
pub struct ArrayBoard {
    /// 0..15 - white piece
    /// 16..31 - black piece
    /// 42 - the Awnser to the Universe
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
                            Some(p) => {
                                if p.is_black() {
                                    format!(" {} ", p.short_name()).black()
                                } else {
                                    format!(" {} ", p.short_name()).bright_white()
                                }
                            }
                        }
                        .on_bright_black()
                    } else {
                        match self.get_piece_on_square((h, v).try_into().unwrap()) {
                            None => "   ".normal(),
                            Some(p) => {
                                if p.is_black() {
                                    format!(" {} ", p.short_name()).black()
                                } else {
                                    format!(" {} ", p.short_name()).bright_white()
                                }
                            }
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
        Self { board: [[None; 8]; 8] }
    }
}

impl Board for ArrayBoard {
    fn get_piece_on_square(&self, square: Square) -> Option<Piece> {
        None
    }

    /// overwrites the previous value at *square* with *piece*
    fn put_piece_option(&mut self, square: Square, piece: Option<Piece>) {}

    /// sets the value at *square* to `None`
    fn clear_square(&mut self, square: Square) {}

    fn get_white_pieces(&self) -> [Option<(Piece, Square)>; 16] {
        [None; 16]
    }
    fn get_black_pieces(&self) -> [Option<(Piece, Square)>; 16] {
        [None; 16]
    }
}
