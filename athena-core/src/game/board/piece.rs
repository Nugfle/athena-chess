//! Chess piece and color representation.
//!
//! This module defines the fundamental types for representing chess pieces
//! and their colors. It provides:
//! - All standard chess pieces with their properties
//! - Color representation with helper methods
//! - Display formatting for algebraic notation
//!
//! # Examples
//!
//! ```rust
//! use athena_core::game::board::piece::{Piece, Color};
//!
//! let king = Piece::King { has_moved: false };
//! assert!(king.is_king());
//!
//! let white = Color::White;
//! let black = !white; // Toggle color
//! ```

use std::{fmt::Display, ops::Not};

/// Represents a chess piece with its type and relevant state.
///
/// Each variant represents a different chess piece type:
/// - Basic pieces (Pawn, Knight, Bishop, Queen)
/// - Stateful pieces (King, Rook) that track movement for castling
///
/// The enum provides methods to:
/// - Query piece type
/// - Update movement state
/// - Format pieces for display
///
/// # Examples
///
/// ```rust
/// use athena_core::game::board::piece::Piece;
///
/// let mut rook = Piece::Rook { has_moved: false };
/// rook.make_moved();
/// assert!(rook.is_rook());
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook { has_moved: bool },
    Queen,
    King { has_moved: bool },
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Pawn => "",
                Self::Knight => "N",
                Self::Bishop => "B",
                Self::Rook { .. } => "R",
                Self::Queen => "Q",
                Self::King { .. } => "K",
            }
        )
    }
}

impl Piece {
    pub fn is_pawn(&self) -> bool {
        *self == Self::Pawn
    }
    pub fn is_knight(&self) -> bool {
        *self == Self::Knight
    }
    pub fn is_bishop(&self) -> bool {
        *self == Self::Bishop
    }
    pub fn is_rook(&self) -> bool {
        matches!(self, Self::Rook { .. })
    }
    pub fn is_queen(&self) -> bool {
        *self == Self::Queen
    }
    pub fn is_king(&self) -> bool {
        matches!(self, Self::King { .. })
    }

    pub fn make_moved(&mut self) {
        match self {
            Self::King { has_moved } => *has_moved = true,
            Self::Rook { has_moved } => *has_moved = true,
            _ => (),
        }
    }
}

/// Represents the color of a chess piece.
///
/// This enum provides:
/// - Basic color representation (White, Black)
/// - Color toggling through Not implementation
/// - Helper methods for color checking
/// - Display formatting
///
/// # Examples
///
/// ```rust
/// use athena_core::game::board::piece::Color;
///
/// let white = Color::White;
/// assert!(white.is_white());
///
/// let black = !white; // Toggle color
/// assert!(black.is_black());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    /// White pieces (starting on ranks 1-2)
    White,
    /// Black pieces (starting on ranks 7-8)
    Black,
}

impl Color {
    pub fn is_white(&self) -> bool {
        *self == Color::White
    }
    pub fn is_black(&self) -> bool {
        *self == Color::Black
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Color::White => "White",
                Color::Black => "Black",
            }
        )
    }
}

impl Not for Color {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}
