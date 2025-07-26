use log::info;
use std::sync::LazyLock;

use attack_tables::AttackTables;
use board::BitBoard;
pub use board::piece::{Color, Piece};
pub use board::square::*;
pub use chess_move::Move;
use error::IllegalMoveError;

mod attack_tables;
mod board;
mod chess_move;
mod error;
mod evaluation;

static ATTACK_TABLES: LazyLock<AttackTables> = LazyLock::new(|| {
    let start = std::time::Instant::now();
    let at = AttackTables::create_tables();
    let took = start.elapsed().as_millis();
    info!("built attack tables, took {} ms...", took);
    at
});

#[cfg(feature = "benchmark")]
pub fn create_tables() {
    AttackTables::create_tables();
}

#[derive(Debug, Clone)]
pub struct Game {
    board: BitBoard,
    moves: Vec<Move>,
    turn: Color,
}

impl Game {
    pub fn init() -> Self {
        let _ = ATTACK_TABLES;
        Self {
            board: BitBoard::init(),
            moves: Vec::new(),
            turn: Color::White,
        }
    }

    pub fn execute_move(&mut self, mut mv: Move) -> Result<(), IllegalMoveError> {
        if let Some((p, c)) = self.board.get_piece_on_square(mv.get_from()) {
            // make sure the color of the piece in the move matches the player whos turn it is
            if *c != self.turn {
                return Err(IllegalMoveError::NotYourPiece {
                    color: *c,
                    square: mv.get_from(),
                });
            }

            // check whether the move is valid for the type of piece
            match mv.get_piece() {
                Piece::Pawn if *p == Piece::Pawn => todo!("compute on the fly"),
                Piece::King if *p == Piece::King => todo!("compute on the fly"),

                Piece::Knight if *p == Piece::Knight => {
                    if ATTACK_TABLES.get_attack_pattern_knight(mv.get_from()).contains(mv.get_to()) {
                        if let Some((taken_piece, col)) = self.board.get_piece_on_square(mv.get_to()) {
                            if *col == self.turn {
                                return Err(IllegalMoveError::TakesOwnPiece { mv: mv, piece: *taken_piece });
                            }
                        }
                    }
                }

                Piece::Bishop if *p == Piece::Bishop => {
                    if ATTACK_TABLES
                        .get_attack_pattern_bishop(mv.get_from(), self.board.occupancy)
                        .contains(mv.get_to())
                    {
                        if let Some((taken_piece, col)) = self.board.get_piece_on_square(mv.get_to()) {
                            if *col == self.turn {
                                return Err(IllegalMoveError::TakesOwnPiece { mv: mv, piece: *taken_piece });
                            }
                        }
                    }
                }

                Piece::Rook if *p == Piece::Rook => {
                    if ATTACK_TABLES
                        .get_attack_pattern_rook(mv.get_from(), self.board.occupancy)
                        .contains(mv.get_to())
                    {
                        if let Some((taken_piece, col)) = self.board.get_piece_on_square(mv.get_to()) {
                            if *col == self.turn {
                                return Err(IllegalMoveError::TakesOwnPiece { mv: mv, piece: *taken_piece });
                            }
                        }
                    }
                }

                Piece::Queen if *p == Piece::Queen => {
                    if ATTACK_TABLES
                        .get_attack_pattern_queen(mv.get_from(), self.board.occupancy)
                        .contains(mv.get_to())
                    {
                        if let Some((taken_piece, col)) = self.board.get_piece_on_square(mv.get_to()) {
                            if *col == self.turn {
                                return Err(IllegalMoveError::TakesOwnPiece { mv: mv, piece: *taken_piece });
                            }
                        }
                    }
                }

                _ => {
                    return Err(IllegalMoveError::DifferentPiece {
                        expected: mv.get_piece(),
                        found: *p,
                    });
                }
            }
            let (p, c) = self
                .board
                .remove_piece_from_square(mv.get_from())
                .expect("checked that from is Some");

            let takes = self
                .board
                .place_piece_on_square(p, c, mv.get_from())
                .and_then(|(taken, _)| Some(taken));

            mv.set_takes(takes);
            self.moves.push(mv);
            Ok(())
        } else {
            Err(IllegalMoveError::EmptySquare { square: mv.get_from() })
        }
    }
}
