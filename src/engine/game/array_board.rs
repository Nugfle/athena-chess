use super::board::Board;
use super::piece::Piece;
use super::square::Square;
use core::fmt::Display;
use std::usize;

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
        self.board[square.vertical() as usize][square.horizontal() as usize]
    }

    /// overwrites the previous value at *square* with *piece*
    fn put_piece_option(&mut self, square: Square, piece: Option<Piece>) {
        self.board[square.vertical() as usize][square.horizontal() as usize] = piece;
    }

    fn get_all_pieces(&self) -> [Option<(Piece, Square)>; 32] {
        let mut i = 0;
        let mut pieces = [const { None }; 32];
        for v in 0..8 {
            for h in 0..8 {
                if let Some(p) = self.board[v][h] {
                    pieces[i] = Some((p, Square::new(h as u8, v as u8).unwrap()));
                    i += 1;
                }
            }
        }
        pieces
    }

    fn get_white_pieces(&self) -> [Option<(Piece, Square)>; 16] {
        let mut i = 0;
        let mut pieces = [const { None }; 16];
        for v in 0..8 {
            for h in 0..8 {
                if let Some(p) = self.board[v][h] {
                    if p.is_white() {
                        pieces[i] = Some((p, Square::new(h as u8, v as u8).unwrap()));
                        i += 1;
                    }
                }
            }
        }
        pieces
    }
    fn get_black_pieces(&self) -> [Option<(Piece, Square)>; 16] {
        let mut i = 0;
        let mut pieces = [const { None }; 16];
        for v in 0..8 {
            for h in 0..8 {
                if let Some(p) = self.board[v][h] {
                    if p.is_black() {
                        pieces[i] = Some((p, Square::new(h as u8, v as u8).unwrap()));
                        i += 1;
                    }
                }
            }
        }
        pieces
    }
}
