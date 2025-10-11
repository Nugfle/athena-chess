//! Chess game logic implementation providing move generation, execution, and game state management.
//!
//! This module implements the core chess game mechanics including:
//! - Board representation using bitboards
//! - Legal move generation
//! - Move execution (including special moves like en passant and castling)
//! - Game state tracking
//!
//! # Examples
//!
//! ```
//! use athena_core::game::Game;
//!
//! let mut game = Game::init(); // Create new game with starting position
//! let moves = game.get_available_moves(); // Get legal moves
//! if let Some(mv) = moves.first() {
//!     game.execute_move(*mv); // Execute a move
//! }
//! ```

use attack_tables::AttackTables;
use board::BitBoard;
use log::info;
pub use mask::BoardMask;
use std::sync::LazyLock;

pub use board::piece::{Color, Piece};
pub use board::square::*;
pub use chess_move::Move;

mod attack_tables;
mod board;
mod chess_move;
mod error;
mod mask;

#[cfg(test)]
mod move_generation_tests;

static ATTACK_TABLES: LazyLock<AttackTables> = LazyLock::new(|| {
    let start = std::time::Instant::now();
    let at = AttackTables::init_table();
    let took = start.elapsed().as_millis();
    info!("setup attack tables, took {took} ms...");
    at
});

#[cfg(feature = "benchmark")]
pub fn create_tables() {
    AttackTables::init_table();
}

#[derive(Debug, Clone)]
/// Represents the complete state of a chess game.
///
/// This struct maintains the current board position, move history,
/// and tracks whose turn it is to move. It provides methods for
/// move generation and execution, including special moves like
/// castling, en passant, and pawn promotion.
pub struct Game {
    /// The current board state using a bitboard representation
    board: BitBoard,
    /// History of moves played in the game
    moves: Vec<Move>,
    /// The color of the player whose turn it is
    turn: Color,
}

impl Game {
    /// Creates a new chess game with the standard starting position.
    ///
    /// The game is initialized with:
    /// - All pieces in their standard starting squares
    /// - White to move first
    /// - Empty move history
    ///
    /// # Examples
    ///
    /// ```
    /// use athena_core::game::Game;
    ///
    /// let game = Game::init();
    /// ```
    pub fn init() -> Self {
        info!("{:?}", ATTACK_TABLES.get_attack_pattern_knight(A2));
        Self {
            board: BitBoard::init(),
            moves: Vec::new(),
            turn: Color::White,
        }
    }

    /// Executes a chess move on the board and updates the game state.
    ///
    /// This method handles all types of moves including:
    /// - Normal moves
    /// - Captures
    /// - En passant captures
    /// - Pawn promotions
    /// - Castling (both kingside and queenside)
    ///
    /// After executing the move:
    /// - The move is added to the move history
    /// - The turn is switched to the other player
    ///
    /// # Arguments
    /// * `mv` - The move to execute
    ///
    /// # Panics
    /// * If trying to move a piece from an empty square
    /// * If trying to castle with missing king or rook
    pub fn execute_move(&mut self, mv: Move) {
        match mv {
            Move::Normal { piece: _, from, to } => {
                let (mut piece, color) = self
                    .board
                    .remove_piece_from_square(from)
                    .expect("from square must contain a piece");
                piece.make_moved();
                self.board.place_piece_on_square(piece, color, to);
            }
            Move::Capture {
                piece: _,
                from,
                to,
                captured: _,
            } => {
                let (mut piece, color) = self
                    .board
                    .remove_piece_from_square(from)
                    .expect("from square must contain a piece");
                piece.make_moved();
                self.board.place_piece_on_square(piece, color, to);
            }
            Move::EnPassant { from, to } => {
                let (piece, color) = self
                    .board
                    .remove_piece_from_square(from)
                    .expect("from square must contain a pawn");
                assert!(self.board.get_piece_on_square(to).is_none(), "to square must be empty");
                self.board.place_piece_on_square(piece, color, to);

                // Remove the captured pawn from the square beside the destination
                let captured_pawn_square = Square::from_rank_file(from.get_rank(), to.get_file());
                self.board
                    .remove_piece_from_square(captured_pawn_square)
                    .expect("captured pawn must exist");
            }
            Move::Promotion { from, to, promoted_to } => {
                let (_, color) = self
                    .board
                    .remove_piece_from_square(from)
                    .expect("from square must contain a pawn");
                let mut promoted_piece = promoted_to;
                promoted_piece.make_moved();
                self.board.place_piece_on_square(promoted_piece, color, to);
            }
            Move::PromotionCapture {
                from,
                to,
                captured: _,
                promoted_to,
            } => {
                let (_, color) = self
                    .board
                    .remove_piece_from_square(from)
                    .expect("from square must contain a pawn");
                let mut promoted_piece = promoted_to;
                promoted_piece.make_moved();
                self.board.place_piece_on_square(promoted_piece, color, to);
            }
            Move::CastleKingside {
                king_from,
                king_to,
                rook_from,
                rook_to,
            } => {
                // Move the king
                let (mut king, color) = self
                    .board
                    .remove_piece_from_square(king_from)
                    .expect("king must be on king_from square");
                king.make_moved();
                self.board.place_piece_on_square(king, color, king_to);

                // Move the rook
                let (mut rook, _) = self
                    .board
                    .remove_piece_from_square(rook_from)
                    .expect("rook must be on rook_from square");
                rook.make_moved();
                self.board.place_piece_on_square(rook, color, rook_to);
            }
            Move::CastleQueenside {
                king_from,
                king_to,
                rook_from,
                rook_to,
            } => {
                // Move the king
                let (mut king, color) = self
                    .board
                    .remove_piece_from_square(king_from)
                    .expect("king must be on king_from square");
                king.make_moved();
                self.board.place_piece_on_square(king, color, king_to);

                // Move the rook
                let (mut rook, _) = self
                    .board
                    .remove_piece_from_square(rook_from)
                    .expect("rook must be on rook_from square");
                rook.make_moved();
                self.board.place_piece_on_square(rook, color, rook_to);
            }
        }

        self.moves.push(mv);
        self.turn = !self.turn;
    }

    /// Returns all legal moves available for the current player.
    ///
    /// This method generates all possible legal moves for the player whose turn it is,
    /// taking into account:
    /// - The current board position
    /// - Special moves like castling and en passant
    /// - Move restrictions (e.g., pinned pieces)
    ///
    /// # Returns
    /// A vector containing all legal moves in the current position
    ///
    /// # Examples
    ///
    /// ```
    /// use athena_core::game::Game;
    ///
    /// let game = Game::init();
    /// let moves = game.get_available_moves();
    /// println!("Number of legal moves: {}", moves.len());
    /// ```
    pub(crate) fn get_available_moves(&self) -> Vec<Move> {
        let pieces: Vec<(Square, Piece)> = self
            .board
            .board
            .iter()
            .enumerate()
            .filter_map(|(i, item)| {
                item.and_then(|(pc, col)| {
                    if col == self.turn {
                        Some((Square::try_from(i).unwrap(), pc))
                    } else {
                        None
                    }
                })
            })
            .collect();

        pieces
            .iter()
            .map(|(sq, pc)| match pc {
                Piece::Rook { .. } => {
                    self.moves_from_mask_and_starting_square(&ATTACK_TABLES.get_attack_pattern_rook(*sq, self.board.occupancy), *sq)
                }
                Piece::Pawn => self.get_pawn_moves(*sq),
                Piece::King { .. } => self.get_king_moves(*sq),
                Piece::Knight => self.moves_from_mask_and_starting_square(&ATTACK_TABLES.get_attack_pattern_knight(*sq), *sq),
                Piece::Queen => self.moves_from_mask_and_starting_square(&ATTACK_TABLES.get_attack_pattern_queen(*sq, self.board.occupancy), *sq),
                Piece::Bishop => self.moves_from_mask_and_starting_square(&ATTACK_TABLES.get_attack_pattern_bishop(*sq, self.board.occupancy), *sq),
            })
            .flatten()
            .collect()
    }

    /// Returns the last move played in the game, if any.
    ///
    /// This is particularly useful for checking en passant possibilities
    /// and other move generation that depends on the previous move.
    ///
    /// # Returns
    /// * `Some(&Move)` - Reference to the last move played
    /// * `None` - If no moves have been played yet
    pub(crate) fn previous_move(&self) -> Option<&Move> {
        self.moves.last()
    }

    /// Generates moves for a piece based on its attack pattern mask.
    ///
    /// This helper method is used by pieces that move in predefined patterns
    /// (Rook, Bishop, Queen, Knight) to convert their attack masks into actual moves.
    ///
    /// # Arguments
    /// * `board_mask` - The mask of squares the piece can potentially move to
    /// * `starting_square` - The current square of the piece
    ///
    /// # Returns
    /// A vector of valid moves for the piece
    fn moves_from_mask_and_starting_square(&self, board_mask: &BoardMask, starting_square: Square) -> Vec<Move> {
        board_mask
            .as_squares()
            .iter()
            .filter_map(|to| match self.board.get_piece_on_square(*to) {
                Some((piece, col)) => {
                    if *col == self.turn {
                        None
                    } else {
                        Some(Move::new(
                            self.board.get_piece_on_square(starting_square).unwrap().0,
                            starting_square,
                            *to,
                            Some(*piece),
                        ))
                    }
                }
                None => Some(Move::new(
                    self.board.get_piece_on_square(starting_square).unwrap().0,
                    starting_square,
                    *to,
                    None,
                )),
            })
            .collect()
    }

    /// Generates all legal moves for a king from its current position.
    ///
    /// This method considers:
    /// - Normal king moves (one square in any direction)
    /// - Castling possibilities (to be implemented)
    /// - Squares under attack by enemy pieces (to be implemented)
    ///
    /// # Arguments
    /// * `from` - The current square of the king
    ///
    /// # Returns
    /// A vector of legal moves for the king
    ///
    /// # Panics
    /// * If the source square doesn't contain a king
    fn get_king_moves(&self, from: Square) -> Vec<Move> {
        let (piece, color) = self.board.get_piece_on_square(from).expect("from must contain a piece");
        assert!(piece.is_king(), "the piece must be a king");
        let mut available_moves = Vec::new();
        todo!("implement king moves");
        available_moves
    }

    /// Generates all legal moves for a pawn from its current position.
    ///
    /// This method handles all special pawn moves:
    /// - Single square advances
    /// - Initial two-square advances
    /// - Diagonal captures
    /// - En passant captures
    /// - Promotions
    ///
    /// # Arguments
    /// * `from` - The current square of the pawn
    ///
    /// # Returns
    /// A vector of legal moves for the pawn
    ///
    /// # Panics
    /// * If the source square doesn't contain a pawn
    /// * If a pawn is found on the back rank
    fn get_pawn_moves(&self, from: Square) -> Vec<Move> {
        let (piece, color) = self.board.get_piece_on_square(from).expect("from must contain a piece");
        assert!(piece.is_pawn(), "the piece must be a pawn");
        let heading = if color.is_white() { 1 } else { -1 };
        let mut available_moves = Vec::new();

        if let Ok(forward) = from.move_on_file(heading) {
            // Handle single square advance
            if self.board.get_piece_on_square(forward).is_none() {
                if forward.get_rank().is_on_edge() {
                    available_moves.extend(Move::promotions(from, forward, None));
                } else {
                    available_moves.push(Move::new(Piece::Pawn, from, forward, None));
                }
                // Handle initial two-square advance
                if (from.get_rank() == Rank::Two && *color == Color::White) || (from.get_rank() == Rank::Seven && *color == Color::Black) {
                    if self
                        .board
                        .get_piece_on_square(forward.move_on_file(heading).expect("we checked"))
                        .is_none()
                    {
                        available_moves.push(Move::new(
                            Piece::Pawn,
                            from,
                            forward.move_on_file(heading).expect("we checked"),
                            None,
                        ));
                    }
                }
            }

            // Handle diagonal captures
            if let Ok(forward_pos) = forward.move_on_rank(1) {
                self.pawn_capture(&mut available_moves, from, forward_pos, color);
            }
            if let Ok(forward_neg) = forward.move_on_rank(-1) {
                self.pawn_capture(&mut available_moves, from, forward_neg, color);
            }

            // Handle en passant
            if self.previous_move().is_some_and(|m| {
                m.get_piece().is_pawn()
                    && m.get_from().get_delta_rank(m.get_to()).abs() == 2
                    && m.get_to().get_delta_rank(from) == 0
                    && m.get_to().get_delta_file(from).abs() == 1
            }) {
                match self.previous_move().expect("checked").get_to().get_delta_file(from) {
                    1 => available_moves.push(Move::en_pasante(from, forward.move_on_rank(-1).expect("checked"))),
                    -1 => available_moves.push(Move::en_pasante(from, forward.move_on_rank(1).expect("checked"))),
                    _ => {}
                }
            }
        } else {
            panic!("error Pawn on the backrank: {}", from);
        }
        available_moves
    }

    /// Helper method to handle pawn capture moves, including promotions.
    ///
    /// # Arguments
    /// * `available_moves` - Vector to store the generated moves
    /// * `from` - The current square of the pawn
    /// * `to` - The target square for the capture
    /// * `color` - The color of the pawn making the capture
    ///
    /// This method will add appropriate moves to the available_moves vector:
    /// - Normal captures
    /// - Capture promotions when reaching the opponent's back rank
    fn pawn_capture(&self, available_moves: &mut Vec<Move>, from: Square, to: Square, color: &Color) {
        match self.board.get_piece_on_square(to) {
            None => {}
            Some((pc, col)) if col != color => {
                if to.get_rank().is_on_edge() {
                    available_moves.extend(Move::promotions(from, to, Some(*pc)));
                } else {
                    available_moves.push(Move::new(Piece::Pawn, from, to, Some(*pc)));
                }
            }
            _ => {}
        }
    }
}
