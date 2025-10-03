//! Chess move representation and manipulation.
//!
//! This module provides a comprehensive representation of chess moves through
//! the `Move` enum, which handles all types of chess moves including:
//! - Normal moves and captures
//! - Special moves (en passant, castling)
//! - Pawn promotions (with and without capture)
//!
//! # Examples
//!
//! ```rust
//! use athena_core::game::{Move, Square, Piece};
//!
//! // Create a normal move
//! let normal = Move::new(Piece::Knight, Square::E4, Square::F6, None);
//!
//! // Create a capture
//! let capture = Move::new(Piece::Bishop, Square::C4, Square::F7, Some(Piece::Pawn));
//!
//! // Create a promotion
//! let promotion = Move::promotion(Square::E7, Square::E8, None, Piece::Queen);
//!
//! // Create a castling move
//! let castle = Move::castle_kingside(Square::E1, Square::G1, Square::H1, Square::F1);
//! ```
//!
//! # Move Validation
//!
//! The module focuses on move representation rather than validation.
//! Move validation is handled by the board logic that generates legal moves.

use std::fmt::Display;

use super::board::piece::Piece;
use super::board::square::Square;

/// Represents all possible chess moves with their specific properties.
///
/// This enum covers every type of chess move:
/// - Normal moves (piece movement to an empty square)
/// - Captures (taking an opponent's piece)
/// - Special pawn moves (en passant)
/// - Pawn promotions (with and without capture)
/// - Castling (both kingside and queenside)
///
/// Each variant contains all necessary information to:
/// - Execute the move on the board
/// - Display the move in algebraic notation
/// - Undo the move if needed
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Move {
    /// Normal move without capture
    Normal {
        piece: Piece,
        from: Square,
        to: Square,
    },
    /// Move that captures an opponent's piece
    Capture {
        piece: Piece,
        from: Square,
        to: Square,
        captured: Piece,
    },
    /// En passant capture
    EnPassant {
        from: Square,
        to: Square,
    },
    /// Pawn promotion without capture
    Promotion {
        from: Square,
        to: Square,
        promoted_to: Piece,
    },
    /// Pawn promotion with capture
    PromotionCapture {
        from: Square,
        to: Square,
        captured: Piece,
        promoted_to: Piece,
    },
    /// Kingside castling
    CastleKingside {
        king_from: Square,
        king_to: Square,
        rook_from: Square,
        rook_to: Square,
    },
    /// Queenside castling
    CastleQueenside {
        king_from: Square,
        king_to: Square,
        rook_from: Square,
        rook_to: Square,
    },
}
impl Move {
    /// Create a new normal move or capture based on whether a piece is taken
    pub fn new(piece: Piece, from: Square, to: Square, takes: Option<Piece>) -> Self {
        match takes {
            Some(captured) => Self::Capture {
                piece,
                from,
                to,
                captured,
            },
            None => Self::Normal { piece, from, to },
        }
    }

    /// Create an en passant move
    pub fn en_pasante(from: Square, to: Square) -> Self {
        Self::EnPassant { from, to }
    }

    /// Create a promotion move (with or without capture)
    pub fn promotion(from: Square, to: Square, takes: Option<Piece>, promotion: Piece) -> Self {
        match takes {
            Some(captured) => Self::PromotionCapture {
                from,
                to,
                captured,
                promoted_to: promotion,
            },
            None => Self::Promotion {
                from,
                to,
                promoted_to: promotion,
            },
        }
    }

    /// Create all possible promotion moves for a pawn
    pub fn promotions(from: Square, to: Square, takes: Option<Piece>) -> [Self; 4] {
        [
            Self::promotion(from, to, takes, Piece::Knight),
            Self::promotion(from, to, takes, Piece::Bishop),
            Self::promotion(from, to, takes, Piece::Rook { has_moved: true }),
            Self::promotion(from, to, takes, Piece::Queen),
        ]
    }

    /// Create a kingside castle move
    pub fn castle_kingside(king_from: Square, king_to: Square, rook_from: Square, rook_to: Square) -> Self {
        Self::CastleKingside {
            king_from,
            king_to,
            rook_from,
            rook_to,
        }
    }

    /// Create a queenside castle move
    pub fn castle_queenside(king_from: Square, king_to: Square, rook_from: Square, rook_to: Square) -> Self {
        Self::CastleQueenside {
            king_from,
            king_to,
            rook_from,
            rook_to,
        }
    }

    /// Get the source square of the move
    pub fn get_from(&self) -> Square {
        match self {
            Self::Normal { from, .. } => *from,
            Self::Capture { from, .. } => *from,
            Self::EnPassant { from, .. } => *from,
            Self::Promotion { from, .. } => *from,
            Self::PromotionCapture { from, .. } => *from,
            Self::CastleKingside { king_from, .. } => *king_from,
            Self::CastleQueenside { king_from, .. } => *king_from,
        }
    }

    /// Get the destination square of the move
    pub fn get_to(&self) -> Square {
        match self {
            Self::Normal { to, .. } => *to,
            Self::Capture { to, .. } => *to,
            Self::EnPassant { to, .. } => *to,
            Self::Promotion { to, .. } => *to,
            Self::PromotionCapture { to, .. } => *to,
            Self::CastleKingside { king_to, .. } => *king_to,
            Self::CastleQueenside { king_to, .. } => *king_to,
        }
    }

    /// Get the piece being moved
    pub fn get_piece(&self) -> Piece {
        match self {
            Self::Normal { piece, .. } => *piece,
            Self::Capture { piece, .. } => *piece,
            Self::EnPassant { .. } => Piece::Pawn,
            Self::Promotion { .. } => Piece::Pawn,
            Self::PromotionCapture { .. } => Piece::Pawn,
            Self::CastleKingside { .. } => Piece::King { has_moved: false },
            Self::CastleQueenside { .. } => Piece::King { has_moved: false },
        }
    }

    /// Get the captured piece if any
    pub fn get_captured(&self) -> Option<Piece> {
        match self {
            Self::Normal { .. } => None,
            Self::Capture { captured, .. } => Some(*captured),
            Self::EnPassant { .. } => Some(Piece::Pawn),
            Self::Promotion { .. } => None,
            Self::PromotionCapture { captured, .. } => Some(*captured),
            Self::CastleKingside { .. } => None,
            Self::CastleQueenside { .. } => None,
        }
    }

    /// Get the promotion piece if this is a promotion move
    pub fn get_promotion(&self) -> Option<Piece> {
        match self {
            Self::Promotion { promoted_to, .. } => Some(*promoted_to),
            Self::PromotionCapture { promoted_to, .. } => Some(*promoted_to),
            _ => None,
        }
    }

    /// Check if this is an en passant move
    pub fn is_en_passant(&self) -> bool {
        matches!(self, Self::EnPassant { .. })
    }

    /// Check if this is a castling move
    pub fn is_castle(&self) -> bool {
        matches!(self, Self::CastleKingside { .. } | Self::CastleQueenside { .. })
    }

    /// Check if this is a promotion move
    pub fn is_promotion(&self) -> bool {
        matches!(self, Self::Promotion { .. } | Self::PromotionCapture { .. })
    }

    /// Sets the captured piece for moves that can capture (for compatibility)
    /// This method is kept for backward compatibility but is less useful with the enum design
    pub fn set_takes(&mut self, piece: Option<Piece>) {
        if let Some(captured) = piece {
            match *self {
                Self::Normal { piece: p, from, to } => {
                    *self = Self::Capture {
                        piece: p,
                        from,
                        to,
                        captured,
                    };
                }
                Self::Promotion { from, to, promoted_to } => {
                    *self = Self::PromotionCapture {
                        from,
                        to,
                        captured,
                        promoted_to,
                    };
                }
                _ => {
                    // For other cases, we don't change the move type
                    // This preserves the existing behavior where set_takes might not always work
                }
            }
        }
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Normal { piece, from, to } => {
                write!(f, "{}{}{}", piece, from, to)
            }
            Self::Capture { piece, from, to, .. } => {
                write!(f, "{}{}x{}", piece, from, to)
            }
            Self::EnPassant { from, to } => {
                write!(f, "{}x{} e.p.", from, to)
            }
            Self::Promotion { from, to, promoted_to } => {
                write!(f, "{}{}={}", from, to, promoted_to)
            }
            Self::PromotionCapture { from, to, promoted_to, .. } => {
                write!(f, "{}x{}={}", from, to, promoted_to)
            }
            Self::CastleKingside { .. } => {
                write!(f, "O-O")
            }
            Self::CastleQueenside { .. } => {
                write!(f, "O-O-O")
            }
        }
    }
}
