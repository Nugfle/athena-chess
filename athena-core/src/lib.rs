//! Athena Chess Engine Core Library
//!
//! `athena-core` is a high-performance chess engine library that provides:
//! - Fast and accurate chess move generation
//! - Position evaluation
//! - Board representation using bitboards
//! - Move validation and execution
//!
//! # Architecture
//!
//! The library is organized into two main modules:
//! - `game`: Core chess logic including move generation and board representation
//! - `evaluation`: Position evaluation and analysis
//!
//! # Examples
//!
//! ```rust
//! use athena_core::game::{Game, Move};
//!
//! // Create a new game
//! let mut game = Game::init();
//!
//! // Get available moves
//! let moves = game.get_available_moves();
//!
//! // Make a move
//! if let Some(mv) = moves.first() {
//!     game.execute_move(*mv);
//! }
//! ```
//!
//! # Features
//!
//! - Efficient bitboard-based move generation
//! - Full support for special moves (castling, en passant, promotions)
//! - Position evaluation using modern chess principles
//! - Support for benchmarking and performance testing
//!
//! # Performance
//!
//! The engine uses several optimization techniques:
//! - Magic bitboard move generation
//! - Pre-calculated attack tables
//! - Efficient board representation
//!
//! Enable the `benchmark` feature to run performance tests.

#![allow(dead_code)]

pub mod evaluation;
pub mod game;
