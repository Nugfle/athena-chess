//! Error types for chess move validation and execution.
//!
//! This module provides a comprehensive error handling system for chess operations:
//! - Move validation errors
//! - Square validation errors
//! - Game rule violation errors
//!
//! The errors are designed to be:
//! - Descriptive (providing detailed error context)
//! - Type-safe (using enums for error categories)
//! - Easy to handle (implementing standard error traits)
//!
//! # Examples
//!
//! ```rust
//! use athena_core::game::error::{ChessError, IllegalMoveError};
//! use athena_core::game::{Move, Square, Color};
//!
//! // Handle a chess error
//! fn handle_move(result: Result<(), ChessError>) {
//!     match result {
//!         Err(ChessError::IllegalMove { e: IllegalMoveError::IsInCheck }) => {
//!             println!("Can't make that move - your king is in check!");
//!         }
//!         Err(e) => println!("Invalid move: {}", e),
//!         Ok(_) => println!("Move successful"),
//!     }
//! }
//! ```

use thiserror::Error;

use super::board::piece::{Color, Piece};
use super::board::square::Square;
use super::chess_move::Move;

/// Top-level error type for chess operations.
///
/// This enum provides high-level categorization of errors that can occur
/// during chess operations. It wraps more specific error types like
/// `IllegalMoveError` for detailed error handling.
#[derive(Debug, Clone, Copy, Error)]
pub enum ChessError {
    #[error("Invalid Square: {square}")]
    InvalidSquare { square: u8 },

    #[error("Illegal Move: {e}")]
    IllegalMove { e: IllegalMoveError },
}

/// Detailed error type for illegal chess moves.
///
/// This enum provides specific error cases for moves that violate chess rules.
/// Each variant includes relevant context (squares, pieces, moves) to help
/// understand and handle the error appropriately.
///
/// The errors cover:
/// - Basic move validation (empty squares, wrong pieces)
/// - Game rule violations (moving into check, blocked paths)
/// - Piece-specific rules (pawn captures, color restrictions)
#[derive(Debug, Clone, Copy, Error, PartialEq, Eq)]
pub enum IllegalMoveError {
    #[error("Empty Square: {square}.")]
    EmptySquare { square: Square },

    #[error("Your King is in Check.")]
    IsInCheck,

    #[error("not a valid move for piece: {mv}.")]
    MoveInvalid { mv: Move },

    #[error("can't move piece on square: {square}. Not your color: {color}.")]
    NotYourPiece { color: Color, square: Square },

    #[error("the piece on the square an the piece in the move don't match. Expected: {expected}, Found: {found}.")]
    DifferentPiece { expected: Piece, found: Piece },

    #[error("the move: {mv}, takes your own piece: {piece}.")]
    TakesOwnPiece { mv: Move, piece: Piece },

    #[error("the move: {mv}, tries to take an empty square: {square} with a pawn")]
    TakesEmptySquare { mv: Move, square: Square },

    #[error("cant do the move: {mv}, the square: {square} is blocked")]
    Blocked { mv: Move, square: Square },
}
