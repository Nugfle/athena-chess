use std::fmt::Display;

pub use array_board::ArrayBoard;
use board::Board;
use chess_move::Move;

mod array_board;
mod board;
mod chess_move;
mod error;
mod piece;
mod square;

/// the representation of a games state, containing all the moves made together with the active color
/// and current state of the Board.
#[derive(Debug, Clone)]
pub struct Game<T>
where
    T: Sized + Board,
{
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
        };
    }
}

impl<T> Game<T>
where
    T: Sized + Board,
{
    pub fn get_legal_moves(&self) -> Vec<Move> {
        // 0 - white 1 - black
        let turn = self.moves.len() % 2;
        let mut legal_moves = Vec::new();
        let pieces = if turn == 0 {
            self.board.get_white_pieces()
        } else {
            self.board.get_black_pieces()
        };

        legal_moves
    }

    pub fn white_is_check(&self) -> bool {
        todo!()
    }
    pub fn black_is_check(&self) -> bool {
        todo!()
    }
}
