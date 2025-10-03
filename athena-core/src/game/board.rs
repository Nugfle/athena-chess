//! Chess board representation using bitboards and piece lists.
//!
//! This module provides an efficient board representation that combines:
//! - A piece list for quick piece placement and removal
//! - A bitboard for fast move generation and attack detection
//! - Methods for board manipulation and state queries
//!
//! # Board Representation
//!
//! The board uses two main data structures:
//! - `board`: An array of 64 squares storing piece types and colors
//! - `occupancy`: A bitboard tracking piece locations for fast lookups
//!
//! # Examples
//!
//! ```rust
//! use athena_core::game::board::{BitBoard, Square, Piece, Color};
//!
//! let mut board = BitBoard::init(); // Create standard chess position
//!
//! // Place and remove pieces
//! let e4 = Square::E4;
//! board.place_piece_on_square(Piece::Queen, Color::White, e4);
//! let piece = board.remove_piece_from_square(e4);
//!
//! // Query the board
//! if board.is_occupied(e4) {
//!     println!("Square is occupied!");
//! }
//! ```

pub mod piece;
pub mod square;

use piece::{Color, Piece};
use square::*;

use crate::game::ATTACK_TABLES;

/// A bitboard representing piece occupancy on the chess board.
///
/// Each bit in the 64-bit integer corresponds to a square on the chess board,
/// where 1 indicates a piece is present and 0 indicates an empty square.
/// This representation allows for extremely fast operations:
/// - Checking if a square is occupied (single bit test)
/// - Finding blocking pieces (bitwise AND with a mask)
/// - Updating piece positions (bitwise OR/AND)
///
/// # Examples
///
/// ```rust
/// use athena_core::game::board::{Occupancy, Square};
///
/// let mut occ = Occupancy::default(); // Empty board
/// let e4 = Square::E4;
///
/// // Add and remove pieces
/// occ.add_square(e4);
/// assert!(occ.is_occupied(e4));
///
/// occ.remove_square(e4);
/// assert!(!occ.is_occupied(e4));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Occupancy(pub u64);

impl Occupancy {
    /// marks the given square as occupied
    pub fn add_square(&mut self, square: Square) {
        self.0 |= 1_u64 << square.as_u8();
    }
    /// returns occupancy with the given square marked as occupied
    pub fn with_square(&self, square: Square) -> Self {
        Occupancy(self.0 | 1_u64 << square.as_u8())
    }
    /// marks the given square as free
    pub fn remove_square(&mut self, square: Square) {
        self.0 &= !(1_u64 << square.as_u8());
    }
    #[allow(unused)]
    /// returns occupancy with the given square marked as free
    pub fn with_square_removed(&self, square: Square) -> Self {
        Occupancy(self.0 & !(1_u64 << square.as_u8()))
    }
    /// checks whether the given square has a piece on it
    pub fn is_occupied(&self, square: Square) -> bool {
        self.0 & 1_u64 << square.as_u8() != 0
    }
}

/// Complete representation of a chess position.
///
/// This struct combines two complementary board representations:
/// - A piece list storing the type and color of pieces on each square
/// - A bitboard tracking piece occupancy for fast move generation
///
/// The dual representation allows for both:
/// - Fast piece lookup and manipulation (through the piece list)
/// - Efficient move generation and attack detection (through the bitboard)
///
/// # Fields
///
/// * `board` - Array of 64 squares, each containing an optional piece and color
/// * `occupancy` - Bitboard tracking which squares are occupied (must be kept in sync with `board`)
///
/// # Examples
///
/// ```rust
/// use athena_core::game::board::BitBoard;
///
/// // Create a board with the standard chess starting position
/// let mut board = BitBoard::init();
///
/// // Create an empty board
/// let empty_board = BitBoard::default();
/// ```
///
/// # Implementation Note
///
/// The `occupancy` field must always be kept in sync with the `board` array.
/// Use the provided methods like `place_piece_on_square` and `remove_piece_from_square`
/// to ensure consistency between the two representations.
#[derive(Debug, Clone)]
pub struct BitBoard {
    /// Array representing each square's contents (piece type and color)
    pub board: [Option<(Piece, Color)>; 64],

    /// Bitboard tracking occupied squares for fast move generation
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

    /// returns true if the square is under attack by a piece from the given color. Note that this
    /// function does not check for pins.
    pub fn square_is_controlled_by(&self, square: Square, color: Color) -> bool {
        let rook_pattern = ATTACK_TABLES.get_attack_pattern_rook(square, self.occupancy);
        let knight_pattern = ATTACK_TABLES.get_attack_pattern_knight(square);
        let bishop_pattern = ATTACK_TABLES.get_attack_pattern_bishop(square, self.occupancy);
        // checks for major pieces
        if self.board.iter().enumerate().any(|(i, p)| {
            p.is_some_and(|(piece, col)| {
                let s = Square::try_from(i).unwrap();
                col == color
                    && ((rook_pattern.contains(s) && (piece.is_rook() || piece.is_queen()))
                        || (bishop_pattern.contains(s) && piece.is_bishop() || piece.is_queen())
                        || (knight_pattern.contains(s) && piece.is_knight()))
            })
        }) {
            return true;
        }
        // checks for king
        if square.move_on_rank(1).is_ok_and(|s| {
            self.board[s.as_index()].is_some_and(|(piece, col)| piece.is_king() && col == color)
                || s.move_on_file(1).is_ok_and(|sf| {
                    self.board[sf.as_index()].is_some_and(|(piece, col)| {
                        col == color
                            && ((piece.is_king()) || (piece.is_pawn() && color == Color::Black))
                    })
                })
        }) {
            return true;
        }
        if square.move_on_rank(-1).is_ok_and(|s| {
            self.board[s.as_index()].is_some_and(|(piece, col)| piece.is_king() && col == color)
                || s.move_on_file(-1).is_ok_and(|sf| {
                    self.board[sf.as_index()].is_some_and(|(piece, col)| {
                        col == color
                            && ((piece.is_king()) || (piece.is_pawn() && color == Color::White))
                    })
                })
        }) {
            return true;
        }
        if square.move_on_rank(1).is_ok_and(|s| {
            self.board[s.as_index()].is_some_and(|(piece, col)| piece.is_king() && col == color)
                || s.move_on_file(-1).is_ok_and(|sf| {
                    self.board[sf.as_index()].is_some_and(|(piece, col)| {
                        col == color
                            && ((piece.is_king()) || (piece.is_pawn() && color == Color::Black))
                    })
                })
        }) {
            return true;
        }
        if square.move_on_rank(-1).is_ok_and(|s| {
            self.board[s.as_index()].is_some_and(|(piece, col)| piece.is_king() && col == color)
                || s.move_on_file(-1).is_ok_and(|sf| {
                    self.board[sf.as_index()].is_some_and(|(piece, col)| {
                        col == color
                            && ((piece.is_king()) || (piece.is_pawn() && color == Color::White))
                    })
                })
        }) {
            return true;
        }

        false
    }

    fn setup_for_game(&mut self) {
        self.place_piece_on_square(Piece::Rook { has_moved: false }, Color::Black, H8);
        self.place_piece_on_square(Piece::Rook { has_moved: false }, Color::Black, A8);
        self.place_piece_on_square(Piece::Knight, Color::Black, G8);
        self.place_piece_on_square(Piece::Knight, Color::Black, B8);
        self.place_piece_on_square(Piece::Bishop, Color::Black, F8);
        self.place_piece_on_square(Piece::Bishop, Color::Black, C8);
        self.place_piece_on_square(Piece::King { has_moved: false }, Color::Black, E8);
        self.place_piece_on_square(Piece::Queen, Color::Black, D8);

        self.place_piece_on_square(Piece::Rook { has_moved: false }, Color::White, H1);
        self.place_piece_on_square(Piece::Rook { has_moved: false }, Color::White, A1);
        self.place_piece_on_square(Piece::Knight, Color::White, G1);
        self.place_piece_on_square(Piece::Knight, Color::White, B1);
        self.place_piece_on_square(Piece::Bishop, Color::White, F1);
        self.place_piece_on_square(Piece::Bishop, Color::White, C1);
        self.place_piece_on_square(Piece::King { has_moved: false }, Color::White, E1);
        self.place_piece_on_square(Piece::Queen, Color::White, D1);

        self.place_piece_on_square(Piece::Pawn, Color::Black, H7);
        self.place_piece_on_square(Piece::Pawn, Color::Black, G7);
        self.place_piece_on_square(Piece::Pawn, Color::Black, F7);
        self.place_piece_on_square(Piece::Pawn, Color::Black, E7);
        self.place_piece_on_square(Piece::Pawn, Color::Black, D7);
        self.place_piece_on_square(Piece::Pawn, Color::Black, C7);
        self.place_piece_on_square(Piece::Pawn, Color::Black, B7);
        self.place_piece_on_square(Piece::Pawn, Color::Black, A7);

        self.place_piece_on_square(Piece::Pawn, Color::White, H2);
        self.place_piece_on_square(Piece::Pawn, Color::White, G2);
        self.place_piece_on_square(Piece::Pawn, Color::White, F2);
        self.place_piece_on_square(Piece::Pawn, Color::White, E2);
        self.place_piece_on_square(Piece::Pawn, Color::White, D2);
        self.place_piece_on_square(Piece::Pawn, Color::White, C2);
        self.place_piece_on_square(Piece::Pawn, Color::White, B2);
        self.place_piece_on_square(Piece::Pawn, Color::White, A2);
    }

    pub fn place_piece_on_square(
        &mut self,
        piece: Piece,
        color: Color,
        square: Square,
    ) -> Option<(Piece, Color)> {
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

    pub fn get_piece_on_square_mut(&mut self, square: Square) -> Option<&mut (Piece, Color)> {
        self.board[square.as_index()].as_mut()
    }

    pub fn is_occupied(&self, square: Square) -> bool {
        self.occupancy.is_occupied(square)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_place_piece_on_square() {
        // setup empty board
        let mut bb = BitBoard::default();
        let sq = Square::from_rank_file(Rank::Four, File::E);
        let pc = Piece::Rook { has_moved: true };
        let col = Color::White;
        assert!(bb.place_piece_on_square(pc, col, sq).is_none());
        assert_eq!(bb.board[28], Some((pc, col)));
        assert!(bb.occupancy.is_occupied(sq));
    }
    #[test]
    fn test_remove_piece_from_square() {
        // setup empty board
        let mut bb = BitBoard::default();
        let sq = Square::from_rank_file(Rank::Four, File::E);
        let pc = Piece::Rook { has_moved: true };
        let col = Color::White;
        bb.board[28] = Some((pc, col));
        bb.occupancy.add_square(Square::new(28).unwrap());
        assert_eq!(Some((pc, col)), bb.remove_piece_from_square(sq));
        assert!(bb.board[28].is_none());
        assert!(!bb.occupancy.is_occupied(sq));
    }
    #[test]
    fn test_get_piece_from_square() {
        // setup empty board
        let mut bb = BitBoard::default();
        let sq = Square::from_rank_file(Rank::Four, File::E);
        let pc = Piece::Rook { has_moved: true };
        let col = Color::White;
        bb.board[28] = Some((pc, col));
        bb.occupancy.add_square(Square::new(28).unwrap());
        assert_eq!(Some(&(pc, col)), bb.get_piece_on_square(sq));
        assert!(bb.board[28].is_some());
        assert!(bb.occupancy.is_occupied(sq));
    }
}
