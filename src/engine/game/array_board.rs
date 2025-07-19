use super::board::Board;
use super::piece::Piece;
use super::square::Square;
use core::fmt::Display;

#[derive(Debug, Clone)]
pub struct ArrayBoard {
    board: [Option<Piece>; 64],
}

impl Default for ArrayBoard {
    /// an empty board, use `init()` to set up all the pieces
    fn default() -> Self {
        Self { board: [None; 64] }
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
        self.board[square.as_index()]
    }

    /// overwrites the previous value at *square* with *piece*
    fn put_piece_option(&mut self, square: Square, piece: Option<Piece>) {
        self.board[square.as_index()] = piece;
    }

    fn get_all_pieces(&self) -> [Option<(Piece, Square)>; 32] {
        let mut pieces = [const { None }; 32];
        self.board.iter().enumerate().for_each(|(i, opt)| {
            let s = Square::try_from(i).unwrap();
            if let Some(p) = opt {
                pieces[i] = Some((*p, s));
            }
        });
        pieces
    }

    fn get_white_pieces(&self) -> [Option<(Piece, Square)>; 16] {
        let mut pieces = [const { None }; 16];
        self.board.iter().enumerate().for_each(|(i, opt)| {
            let s = Square::try_from(i).unwrap();
            if let Some(p) = opt {
                if p.is_white() {
                    pieces[i] = Some((*p, s));
                }
            }
        });
        pieces
    }
    fn get_black_pieces(&self) -> [Option<(Piece, Square)>; 16] {
        let mut pieces = [const { None }; 16];
        self.board.iter().enumerate().for_each(|(i, opt)| {
            let s = Square::try_from(i).unwrap();
            if let Some(p) = opt {
                if p.is_black() {
                    pieces[i] = Some((*p, s));
                }
            }
        });
        pieces
    }
}
