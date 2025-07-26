pub mod piece;
pub mod square;

use piece::{Color, Piece};
use square::*;

/// a representation of the board where each bit in the u64 represents the square on the board and
/// whether it is occupied. This makes checking for blocking pieces as easy as applying a mask to
/// the Occupancy and voila, you get all the squares with blocking pieces
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Occupancy(pub u64);

impl Occupancy {
    pub fn add_square(&mut self, square: Square) {
        self.0 |= 1_u64 << square.as_u8();
    }
    pub fn with_square(&self, square: Square) -> Self {
        Occupancy(self.0 | 1_u64 << square.as_u8())
    }
    pub fn remove_square(&mut self, square: Square) {
        self.0 &= !(1_u64 << square.as_u8());
    }
    pub fn with_square_removed(&self, square: Square) -> Self {
        Occupancy(self.0 & !(1_u64 << square.as_u8()))
    }
    pub fn is_occupied(&self, square: Square) -> bool {
        self.0 & 1_u64 << square.as_u8() != 0
    }
}

/// represents the current Board state.
#[derive(Debug, Clone)]
pub struct BitBoard {
    pub board: [Option<(Piece, Color)>; 64],

    // tracks whether each square is occupied, must be kept in sync with the board. Should only be
    // used for lookups in the Attack Tables.
    pub occupancy: Occupancy,
}

impl Default for BitBoard {
    fn default() -> Self {
        Self {
            board: [None; 64],
            occupancy: Occupancy(0),
        }
    }
}

impl BitBoard {
    pub fn init() -> Self {
        let mut bb = Self::default();
        bb.setup_for_game();
        bb
    }

    fn setup_for_game(&mut self) {
        self.place_piece_on_square(Piece::Rook, Color::Black, H8);
        self.place_piece_on_square(Piece::Rook, Color::Black, A8);
        self.place_piece_on_square(Piece::Knight, Color::Black, G8);
        self.place_piece_on_square(Piece::Knight, Color::Black, B8);
        self.place_piece_on_square(Piece::Bishop, Color::Black, F8);
        self.place_piece_on_square(Piece::Bishop, Color::Black, C8);
        self.place_piece_on_square(Piece::King { can_castle: true }, Color::Black, E8);
        self.place_piece_on_square(Piece::Queen, Color::Black, D8);

        self.place_piece_on_square(Piece::Rook, Color::White, H1);
        self.place_piece_on_square(Piece::Rook, Color::White, A1);
        self.place_piece_on_square(Piece::Knight, Color::White, G1);
        self.place_piece_on_square(Piece::Knight, Color::White, B1);
        self.place_piece_on_square(Piece::Bishop, Color::White, F1);
        self.place_piece_on_square(Piece::Bishop, Color::White, C1);
        self.place_piece_on_square(Piece::King { can_castle: true }, Color::White, E1);
        self.place_piece_on_square(Piece::Queen, Color::White, D1);

        self.place_piece_on_square(Piece::Pawn { en_pasent: false }, Color::Black, H7);
        self.place_piece_on_square(Piece::Pawn { en_pasent: false }, Color::Black, G7);
        self.place_piece_on_square(Piece::Pawn { en_pasent: false }, Color::Black, F7);
        self.place_piece_on_square(Piece::Pawn { en_pasent: false }, Color::Black, E7);
        self.place_piece_on_square(Piece::Pawn { en_pasent: false }, Color::Black, D7);
        self.place_piece_on_square(Piece::Pawn { en_pasent: false }, Color::Black, C7);
        self.place_piece_on_square(Piece::Pawn { en_pasent: false }, Color::Black, B7);
        self.place_piece_on_square(Piece::Pawn { en_pasent: false }, Color::Black, A7);

        self.place_piece_on_square(Piece::Pawn { en_pasent: false }, Color::White, H2);
        self.place_piece_on_square(Piece::Pawn { en_pasent: false }, Color::White, G2);
        self.place_piece_on_square(Piece::Pawn { en_pasent: false }, Color::White, F2);
        self.place_piece_on_square(Piece::Pawn { en_pasent: false }, Color::White, E2);
        self.place_piece_on_square(Piece::Pawn { en_pasent: false }, Color::White, D2);
        self.place_piece_on_square(Piece::Pawn { en_pasent: false }, Color::White, C2);
        self.place_piece_on_square(Piece::Pawn { en_pasent: false }, Color::White, B2);
        self.place_piece_on_square(Piece::Pawn { en_pasent: false }, Color::White, A2);
    }

    pub fn place_piece_on_square(&mut self, piece: Piece, color: Color, square: Square) -> Option<(Piece, Color)> {
        self.occupancy.add_square(square);
        self.board[square.as_index()].replace((piece, color))
    }

    pub fn remove_piece_from_square(&mut self, square: Square) -> Option<(Piece, Color)> {
        self.occupancy.remove_square(square);
        self.board[square.as_index()].take()
    }

    pub fn get_piece_on_square(&self, square: Square) -> Option<&(Piece, Color)> {
        self.board[square.as_index()].as_ref()
    }

    pub fn is_occupied(&self, square: Square) -> bool {
        self.occupancy.is_occupied(square)
    }
}
