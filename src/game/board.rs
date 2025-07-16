use super::piece::{Color, Piece};
use super::square::Square;

pub trait Board: Default {
    // template methods
    fn get_piece_on_square(&self, square: Square) -> Option<Piece>;
    fn put_piece_option(&mut self, square: Square, piece: Option<Piece>);
    fn clear_square(&mut self, square: Square);
    fn get_white_pieces(&self) -> [Option<(Piece, Square)>; 16];
    fn get_black_pieces(&self) -> [Option<(Piece, Square)>; 16];

    fn put_piece(&mut self, square: Square, piece: Piece) {
        self.put_piece_option(square, Some(piece));
    }

    fn init() -> Self {
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

#[cfg(test)]
mod test {}
