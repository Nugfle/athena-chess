//! Bitboard masks for efficient chess move and attack pattern representation.
//!
//! This module provides the `BoardMask` type, which represents a set of squares
//! on a chess board using a 64-bit integer. Each bit corresponds to one square,
//! allowing for efficient:
//! - Move pattern generation
//! - Attack pattern calculation
//! - Square set operations (union, intersection)
//!
//! # Examples
//!
//! ```rust
//! use athena_core::game::mask::BoardMask;
//! use athena_core::game::board::Square;
//!
//! // Create masks
//! let mut mask1 = BoardMask::default();
//! let mut mask2 = BoardMask::default();
//!
//! // Add squares
//! mask1.add_square(Square::E4);
//! mask2.add_square(Square::E5);
//!
//! // Combine masks
//! let combined = mask1 | mask2;
//! let intersection = mask1 & mask2;
//!
//! // Convert to squares
//! let squares: Vec<Square> = combined.as_squares();
//! ```

use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

use serde::{Deserialize, Serialize};

use crate::game::board::square::Square;

/// A bitboard representing a set of squares on a chess board.
///
/// `BoardMask` uses a 64-bit integer where each bit represents one square.
/// The mapping follows standard chess board layout:
/// - Least significant bit: A1
/// - Most significant bit: H8
///
/// This type implements standard bitwise operations to allow efficient
/// set operations on squares:
/// - OR (|): Union of two sets of squares
/// - AND (&): Intersection of two sets
/// - XOR (^): Symmetric difference
/// - NOT (!): Complement (all squares not in the set)
///
/// # Examples
///
/// ```rust
/// use athena_core::game::mask::BoardMask;
///
/// let mut mask = BoardMask::default(); // Empty mask
/// mask.add_square(Square::E4);        // Set E4 bit
/// assert!(mask.contains(Square::E4));  // Check if E4 is set
/// ```
#[derive(Debug, Clone, Default, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct BoardMask(pub u64);

impl BitOr<BoardMask> for BoardMask {
    type Output = BoardMask;
    fn bitor(self, rhs: BoardMask) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}
impl BitAnd<BoardMask> for BoardMask {
    type Output = BoardMask;
    fn bitand(self, rhs: BoardMask) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}
impl BitXor<BoardMask> for BoardMask {
    type Output = BoardMask;
    fn bitxor(self, rhs: BoardMask) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}
impl BitOrAssign<BoardMask> for BoardMask {
    fn bitor_assign(&mut self, rhs: BoardMask) {
        self.0 |= rhs.0
    }
}
impl BitAndAssign<BoardMask> for BoardMask {
    fn bitand_assign(&mut self, rhs: BoardMask) {
        self.0 &= rhs.0
    }
}
impl BitXorAssign<BoardMask> for BoardMask {
    fn bitxor_assign(&mut self, rhs: BoardMask) {
        self.0 ^= rhs.0
    }
}
impl Not for BoardMask {
    type Output = Self;
    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}
impl Into<Vec<Square>> for BoardMask {
    fn into(self) -> Vec<Square> {
        let mut v = Vec::with_capacity(self.0.count_ones() as usize);
        for i in 0..64 {
            if self.0 & (1_u64 << i) != 0 {
                v.push(Square::new(i).unwrap());
            }
        }
        v
    }
}

impl Into<Vec<Square>> for &BoardMask {
    fn into(self) -> Vec<Square> {
        let mut v = Vec::with_capacity(self.0.count_ones() as usize);
        for i in 0..64 {
            if self.0 & (1_u64 << i) != 0 {
                v.push(Square::new(i).unwrap());
            }
        }
        v
    }
}

impl BoardMask {
    pub fn add_square(&mut self, square: Square) {
        self.0 |= 1_u64 << square.as_u8();
    }
    pub fn with_square(&self, square: Square) -> Self {
        Self(self.0 | 1_u64 << square.as_u8())
    }
    pub fn remove_square(&mut self, square: Square) {
        self.0 &= !(1_u64 << square.as_u8());
    }
    pub fn with_square_removed(&self, square: Square) -> Self {
        Self(self.0 & !(1_u64 << square.as_u8()))
    }
    pub fn contains(&self, square: Square) -> bool {
        self.0 & (1_u64 << square.as_u8()) != 0
    }
    pub fn count_ones(&self) -> u32 {
        self.0.count_ones()
    }
    pub fn add_squares(&mut self, squares: impl IntoIterator<Item = Square>) {
        squares.into_iter().for_each(|sq| self.add_square(sq));
    }

    pub fn as_squares(&self) -> Vec<Square> {
        self.into()
    }
}
