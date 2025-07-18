use crate::game::board::Board;
use crate::game::piece::Piece;
use crate::game::square::*;
use core::fmt::Display;

#[derive(Debug, Clone)]
pub struct ArrayBoard {
    board: [[Option<Piece>; 8]; 8],
}
impl Default for ArrayBoard {
    /// an empty board, use `init()` to set up all the pieces
    fn default() -> Self {
        Self { board: [[None; 8]; 8] }
    }
}

impl Display for ArrayBoard {
    /// a pretty printed chess board with the Capital Letters as Pieces
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.format_print_board(f)
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
        let mut i = 0;
        let mut pieces = [const { None }; 16];
        for v in 0..8 {
            for h in 0..8 {
                self.board[v][h]
                    .and_then(|p| if p.is_white() { Some(p) } else { None })
                    .and_then(|p| {
                        pieces[i] = Some((Square::new(h as u8, v as u8), p));
                        i += 1;
                        Some(())
                    });
            }
        }
        [None; 16]
    }
    fn get_black_pieces(&self) -> [Option<(Piece, Square)>; 16] {
        [None; 16]
    }
}
