//! Core move generation logic for chess pieces.
//!
//! This module implements the fundamental move pattern generation for:
//! - Sliding pieces (rooks, bishops)
//! - Non-sliding pieces (knights)
//! - Attack pattern masks for magic bitboard generation
//!
//! The module provides both:
//! - Pattern generation considering blocking pieces (attack patterns)
//! - Pattern generation ignoring blocking pieces (masks for magic calculation)
//!
//! # Implementation Details
//!
//! For sliding pieces (rooks, bishops):
//! - Masks exclude edge squares to reduce pattern count
//! - Attack patterns stop at first blocking piece
//! - Efficient bitboard operations for pattern generation
//!
//! For knights:
//! - Simple pattern generation (no blocking pieces)
//! - Boundary checking for edge cases
//!
//! # Examples
//!
//! ```rust
//! use athena_core::game::attack_tables::move_logic::{
//!     create_rook_attack_pattern,
//!     create_bishop_mask,
//!     create_knight_attack_pattern
//! };
//!
//! // Generate patterns
//! let knight_pattern = create_knight_attack_pattern(Square::E4);
//! let bishop_mask = create_bishop_mask(Square::E4);
//! let rook_attacks = create_rook_attack_pattern(Square::E4, current_occupancy);
//! ```

use crate::game::BoardMask;
use crate::game::board::Occupancy;
use crate::game::board::square::Square;

/// Generates attack pattern for a knight on the given square.
///
/// Knights are non-sliding pieces, so their attack patterns:
/// - Don't depend on blocking pieces
/// - Only need boundary checking
/// - Are simpler to generate than sliding pieces
pub fn create_knight_attack_pattern(square: Square) -> BoardMask {
    let mut pattern = BoardMask(0);
    // -2 -1
    if square > Square::new(17).unwrap() {
        pattern.add_square(Square::new(square.as_u8() - 17).unwrap());
    }
    // -2 + 1
    if square > Square::new(17).unwrap() {
        pattern.add_square(Square::new(square.as_u8() - 15).unwrap());
    }
    // -1, -2
    if square > Square::new(17).unwrap() {
        pattern.add_square(Square::new(square.as_u8() - 10).unwrap());
    }
    // -1, +2
    if square > Square::new(17).unwrap() {
        pattern.add_square(Square::new(square.as_u8() - 6).unwrap());
    }

    // +1 -2
    if square < Square::new(58).unwrap() {
        pattern.add_square(Square::new(square.as_u8() + 6).unwrap());
    }
    // +1 +2
    if square < Square::new(54).unwrap() {
        pattern.add_square(Square::new(square.as_u8() + 10).unwrap());
    }
    // +2 -1
    if square < Square::new(49).unwrap() {
        pattern.add_square(Square::new(square.as_u8() + 15).unwrap());
    }
    // +2 +1
    if square < Square::new(47).unwrap() {
        pattern.add_square(Square::new(square.as_u8() + 17).unwrap());
    }

    pattern
}

/// Generates a mask for rook magic bitboard calculation.
///
/// This mask includes all squares that can affect a rook's movement:
/// - Horizontal and vertical lines from the rook's position
/// - Excludes edge squares (to reduce pattern count)
/// - Used for magic bitboard indexing
///
/// # Examples
///
/// ```rust
/// use athena_core::game::attack_tables::move_logic::create_rook_mask;
///
/// let mask = create_rook_mask(Square::E4);
/// // Mask includes squares that can block rook movement
/// // but excludes edge squares to optimize magic calculation
/// ```
pub fn create_rook_mask(square: Square) -> BoardMask {
    let mut mask = BoardMask(0);
    let mut i = square.as_u8();
    // cover the rows
    while i % 8 < 7 {
        mask.add_square(Square::new(i).unwrap());
        i += 1;
    }
    i = square.as_u8();
    while !i.is_multiple_of(8) {
        mask.add_square(Square::new(i).unwrap());
        i -= 1;
    }
    i = square.as_u8();
    // cover the columns
    while i / 8 < 7 {
        mask.add_square(Square::new(i).unwrap());
        i += 8;
    }
    i = square.as_u8();
    while i / 8 > 0 {
        mask.add_square(Square::new(i).unwrap());
        i -= 8;
    }
    // our current logic adds the starting square, so we just remove it
    mask.with_square_removed(square)
}

/// Generates attack pattern for a rook considering blocking pieces.
///
/// This function:
/// - Calculates all squares a rook can attack
/// - Considers blocking pieces (stops at first blocker)
/// - Includes captures (squares with enemy pieces)
/// - Used for actual move generation
///
/// # Arguments
///
/// * `square` - Starting square of the rook
/// * `occupancy` - Current board occupancy (blocking pieces)
///
/// # Examples
///
/// ```rust
/// use athena_core::game::attack_tables::move_logic::create_rook_attack_pattern;
///
/// let attacks = create_rook_attack_pattern(Square::E4, current_occupancy);
/// // Pattern includes all squares the rook can move to or attack
/// // Stops at first blocking piece in each direction
/// ```
pub fn create_rook_attack_pattern(square: Square, occupancy: Occupancy) -> BoardMask {
    let mut mask = BoardMask(0);

    // can't panic because square is in range 0-63
    let s: i8 = square.as_u8().try_into().unwrap();
    let mut i = s;

    while i / 8 == s / 8 && i < 64 {
        let sq = Square::new(i as u8).unwrap();
        mask.add_square(sq);
        if occupancy.is_occupied(sq) {
            break;
        }
        i += 1;
    }
    i = s;
    while i / 8 == s / 8 && i >= 0 {
        let sq = Square::new(i as u8).unwrap();
        mask.add_square(sq);
        if occupancy.is_occupied(sq) {
            break;
        }
        i -= 1;
    }
    i = s;
    // cover the columns
    while i < 64 {
        let sq = Square::new(i as u8).unwrap();
        mask.add_square(sq);
        if occupancy.is_occupied(sq) {
            break;
        }
        i += 8;
    }
    i = s;
    while i >= 0 {
        let sq = Square::new(i as u8).unwrap();
        mask.add_square(sq);
        if occupancy.is_occupied(sq) {
            break;
        }
        i -= 8;
    }
    // our current logic adds the starting square, so we just remove it
    mask.with_square_removed(square)
}

/// Generates a mask for bishop magic bitboard calculation.
///
/// This mask includes all squares that can affect a bishop's movement:
/// - Diagonal lines from the bishop's position
/// - Excludes edge squares (to reduce pattern count)
/// - Used for magic bitboard indexing
///
/// # Examples
///
/// ```rust
/// use athena_core::game::attack_tables::move_logic::create_bishop_mask;
///
/// let mask = create_bishop_mask(Square::E4);
/// // Mask includes squares that can block bishop movement
/// // but excludes edge squares to optimize magic calculation
/// ```
pub fn create_bishop_mask(square: Square) -> BoardMask {
    let mut mask: BoardMask = BoardMask(0);

    let mut i = square.as_u8();

    // goes into ++ direction until it hits the edge of the board
    while i % 8 < 7 && i / 8 < 7 {
        // can't panic because square is valid by checks above
        mask.add_square(Square::new(i).unwrap());
        i += 9;
    }
    i = square.as_u8();
    // goes into -- direction until it hits the edge of the board
    while !i.is_multiple_of(8) && i / 8 > 0 {
        mask.add_square(Square::new(i).unwrap());
        i -= 9;
    }
    i = square.as_u8();
    // goes into -+ direction until it hits the edge of the board
    while i % 8 < 7 && i / 8 > 0 {
        mask.add_square(Square::new(i).unwrap());
        i -= 7;
    }
    i = square.as_u8();
    // goes into +- direction until it hits the edge of the board
    while !i.is_multiple_of(8) && i / 8 < 7 {
        mask.add_square(Square::new(i).unwrap());
        i += 7;
    }
    // our current logic adds the starting square, so we just remove it
    mask.with_square_removed(square)
}
/// Generates attack pattern for a bishop considering blocking pieces.
///
/// This function:
/// - Calculates all squares a bishop can attack
/// - Considers blocking pieces (stops at first blocker)
/// - Includes captures (squares with enemy pieces)
/// - Used for actual move generation
///
/// # Arguments
///
/// * `square` - Starting square of the bishop
/// * `occupancy` - Current board occupancy (blocking pieces)
///
/// # Examples
///
/// ```rust
/// use athena_core::game::attack_tables::move_logic::create_bishop_attack_pattern;
///
/// let attacks = create_bishop_attack_pattern(Square::E4, current_occupancy);
/// // Pattern includes all squares the bishop can move to or attack
/// // Stops at first blocking piece in each diagonal
/// ```
pub fn create_bishop_attack_pattern(square: Square, occupancy: Occupancy) -> BoardMask {
    let mut mask = BoardMask(0);
    // can't panic because square is always in range 0-63
    let s: i8 = square.as_u8().try_into().unwrap();
    let mut i = s;

    // goes into ++ direction until it hits the edge of the board
    while i < 64 {
        mask.add_square(Square::new(i as u8).unwrap());
        if occupancy.0 & (1 << i) != 0 || i % 8 == 7 {
            break;
        }
        i += 9;
    }
    i = s;
    // goes into -- direction until it hits the edge of the board
    while i >= 0 {
        mask.add_square(Square::new(i as u8).unwrap());
        if occupancy.0 & (1 << i) != 0 || i % 8 == 0 {
            break;
        }
        i -= 9;
    }
    i = s;
    // goes into -+ direction until it hits the edge of the board
    while i >= 0 {
        mask.add_square(Square::new(i as u8).unwrap());
        if occupancy.0 & (1 << i) != 0 || i % 8 == 7 {
            break;
        }
        i -= 7;
    }
    i = s;
    // goes into +- direction until it hits the edge of the board
    while i < 64 {
        mask.add_square(Square::new(i as u8).unwrap());
        if occupancy.0 & (1 << i) != 0 || i % 8 == 0 {
            break;
        }
        i += 7;
    }
    // our current logic adds the starting square, so we just remove it
    mask.with_square_removed(square)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::game::board::square::*;

    fn squares_from_mask(mask: BoardMask) -> Vec<String> {
        let mut squares = Vec::new();
        for i in 0..64 {
            if mask.contains(Square::new(i).unwrap()) {
                squares.push(Square::new(i).unwrap().to_string());
            }
        }
        squares
    }

    fn check_bit_board_pattern(expected: BoardMask, computed: BoardMask) {
        assert_eq!(
            computed,
            expected,
            "assert failed:\nexpected: {:>64b}\ngot:      {:>64b}\nerror:    {:>64b}\nsquares:  {:?}",
            expected.0,
            computed.0,
            (computed ^ expected).0,
            squares_from_mask(computed ^ expected),
        )
    }

    #[test]
    fn test_create_bishop_mask() {
        // put the bishop on d3;
        let s = D4;
        let m = create_bishop_mask(s);
        let expected = BoardMask(0)
            .with_square(E3)
            .with_square(F2)
            .with_square(C3)
            .with_square(B2)
            .with_square(C5)
            .with_square(B6)
            .with_square(E5)
            .with_square(F6)
            .with_square(G7);
        check_bit_board_pattern(expected, m);
    }

    #[test]
    fn test_create_rook_mask() {
        // put the bishop on d3;
        let s = D4;
        let m = create_rook_mask(s);

        let expected = BoardMask(0)
            .with_square(D5)
            .with_square(D6)
            .with_square(D7)
            .with_square(D3)
            .with_square(D2)
            .with_square(C4)
            .with_square(B4)
            .with_square(E4)
            .with_square(F4)
            .with_square(G4);
        check_bit_board_pattern(expected, m);
    }

    #[test]
    fn test_create_bishop_attack_pattern_no_occupants() {
        // put the bishop on d3;
        let s = D4;
        let m = create_bishop_attack_pattern(s, Occupancy(0));

        let expected = BoardMask(0)
            .with_square(E3)
            .with_square(F2)
            .with_square(G1)
            .with_square(C3)
            .with_square(B2)
            .with_square(A1)
            .with_square(C5)
            .with_square(B6)
            .with_square(A7)
            .with_square(E5)
            .with_square(F6)
            .with_square(G7)
            .with_square(H8);

        check_bit_board_pattern(expected, m);
    }
    #[test]
    fn test_create_bishop_attack_pattern_with_occupants() {
        // put the bishop on d3;
        let s = D4;
        let occupancy = Occupancy(0).with_square(F2).with_square(A1).with_square(C5).with_square(F6);
        let m = create_bishop_attack_pattern(s, occupancy);

        let expected = BoardMask(0)
            .with_square(E3)
            .with_square(F2)
            .with_square(C3)
            .with_square(B2)
            .with_square(A1)
            .with_square(C5)
            .with_square(E5)
            .with_square(F6);

        check_bit_board_pattern(expected, m);
    }

    #[test]
    fn test_create_rook_attack_pattern_no_occupants() {
        // put the bishop on d3;
        let s = D4;
        let m = create_rook_attack_pattern(s, Occupancy(0));
        let expected = BoardMask(0)
            .with_square(D5)
            .with_square(D6)
            .with_square(D7)
            .with_square(D8)
            .with_square(D3)
            .with_square(D2)
            .with_square(D1)
            .with_square(C4)
            .with_square(B4)
            .with_square(A4)
            .with_square(E4)
            .with_square(F4)
            .with_square(G4)
            .with_square(H4);
        check_bit_board_pattern(expected, m);
    }
    #[test]
    fn test_create_rook_attack_pattern_with_occupants() {
        // put the bishop on d3;
        let s = D4;
        let occupancy = Occupancy(0)
            .with_square(D7)
            .with_square(D3)
            .with_square(A4)
            .with_square(F4)
            .with_square(H4)
            .with_square(H8);
        let m = create_rook_attack_pattern(s, occupancy);
        let expected = BoardMask(0)
            .with_square(D5)
            .with_square(D6)
            .with_square(D7)
            .with_square(D3)
            .with_square(C4)
            .with_square(B4)
            .with_square(A4)
            .with_square(E4)
            .with_square(F4);
        check_bit_board_pattern(expected, m);
    }
}
