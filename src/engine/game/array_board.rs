use super::board::Board;
use super::piece::Piece;
use super::square::Square;
use core::fmt::Display;

#[derive(Debug, Clone)]
pub struct ArrayBoard {
    board: [Option<Piece>; 64],
}

pub struct ArrayBoardIter<'a> {
    board: &'a [Option<Piece>; 64],
    index: usize,
}

impl<'a> Iterator for ArrayBoardIter<'a> {
    type Item = (Square, Option<&'a Piece>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= 64 {
            return None;
        }
        let s = Square::try_from(self.index).unwrap();
        self.index += 1;
        Some((s, self.board[s.as_index()].as_ref()))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (64 - self.index, Some(64 - self.index))
    }
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
    type Iter<'a>
        = ArrayBoardIter<'a>
    where
        Self: 'a;

    fn iter(&self) -> Self::Iter<'_> {
        ArrayBoardIter {
            board: &self.board,
            index: 0,
        }
    }

    fn get_piece_on_square(&self, square: Square) -> Option<Piece> {
        self.board[square.as_index()]
    }

    /// overwrites the previous value at *square* with *piece*
    fn put_piece_option(&mut self, square: Square, piece: Option<Piece>) {
        self.board[square.as_index()] = piece;
    }

    fn make_move(&mut self, from: Square, to: Square) {
        self.board[to.as_index()] = std::mem::take(&mut self.board[from.as_index()])
    }
}
