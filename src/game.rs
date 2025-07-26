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
        let from = mv.get_from();
        let to = mv.get_to();

        if self.board.get_piece_on_square(from).is_none() {
            return Err(IllegalMoveError::EmptySquare { square: from });
        }

        let (p, c) = self.board.get_piece_on_square(mv.get_from()).unwrap();

        // make sure the color of the piece in the move matches the player whos turn it is
        if *c != self.turn {
            return Err(IllegalMoveError::NotYourPiece { color: *c, square: from });
        }

        // check whether the move is valid for the type of piece
        match mv.get_piece() {
            Piece::Pawn if *p == Piece::Pawn && self.turn == Color::White => {
                // pawn can at most move 2 ranks and only forward
                if from.get_delta_rank(to) > 2 || from.get_delta_rank(to) <= 0 {
                    return Err(IllegalMoveError::MoveInvalid { mv });
                }
                // handle double moves
                if from.get_delta_rank(to) == 2 {
                    // double moves may only happen in a straight line
                    if from.get_delta_file(to) != 0 {
                        return Err(IllegalMoveError::MoveInvalid { mv });
                    }
                    if self.board.is_occupied(from.move_on_file(1).unwrap()) {
                        return Err(IllegalMoveError::Blocked {
                            mv: mv,
                            square: from.move_on_file(1).unwrap(),
                        });
                    }
                    if self.board.is_occupied(to) {
                        return Err(IllegalMoveError::Blocked { mv: mv, square: to });
                    }
                }

                // 1 forward moves
                if from.get_delta_rank(to) == 1 {
                    // pushing pawn 1 square
                    if from.get_delta_file(to) == 0 && self.board.is_occupied(to) {
                        return Err(IllegalMoveError::MoveInvalid { mv });
                    }

                    // takes to the right
                    if from.get_delta_file(to) == 1 {
                        // if the previous move was a double pawn move on the file that we are moving
                        // to and it put the pawn next to us.
                        if self.moves.last().is_some_and(|m| {
                            m.get_piece() == Piece::Pawn
                                && m.get_from().get_delta_rank(m.get_to()).abs() == 2
                                && m.get_to().get_file() == to.get_file()
                                && m.get_to().get_rank() == from.get_rank()
                        }) {
                            info!("en-pasent");
                            self.board.remove_piece_from_square(self.moves.last().unwrap().get_to());
                        } else {
                            // takes to the right
                            match self.board.get_piece_on_square(to) {
                                Some((pc, col)) if *col == self.turn => return Err(IllegalMoveError::TakesOwnPiece { mv: mv, piece: *pc }),
                                None => {
                                    return Err(IllegalMoveError::TakesEmptySquare { mv, square: to });
                                }
                                _ => {}
                            }
                        }
                    }
                    // takes to the left
                    if from.get_delta_file(to) == -1 {
                        if self.moves.last().is_some_and(|m| {
                            m.get_piece() == Piece::Pawn
                                && m.get_from().get_delta_rank(m.get_to()).abs() == 2
                                && m.get_to().get_file() == to.get_file()
                                && m.get_to().get_rank() == from.get_rank()
                        }) {
                            info!("en-pasent");
                            self.board.remove_piece_from_square(self.moves.last().unwrap().get_to());
                        } else {
                            match self.board.get_piece_on_square(to) {
                                Some((pc, col)) if *col == self.turn => return Err(IllegalMoveError::TakesOwnPiece { mv: mv, piece: *pc }),
                                None => {
                                    return Err(IllegalMoveError::TakesEmptySquare { mv, square: to });
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }

            Piece::Pawn if *p == Piece::Pawn && self.turn == Color::Black => {
                // pawn can at most move 2 ranks and only forward
                if from.get_delta_rank(to) < -2 || from.get_delta_rank(to) >= 0 {
                    return Err(IllegalMoveError::MoveInvalid { mv });
                }
                // handle double moves
                if from.get_delta_rank(to) == -2 {
                    // double moves may only happen in a straight line
                    if from.get_delta_file(to) != 0 {
                        return Err(IllegalMoveError::MoveInvalid { mv });
                    }
                    if self.board.is_occupied(from.move_on_file(-1).unwrap()) {
                        return Err(IllegalMoveError::Blocked {
                            mv: mv,
                            square: from.move_on_file(-1).unwrap(),
                        });
                    }
                    if self.board.is_occupied(to) {
                        return Err(IllegalMoveError::Blocked { mv: mv, square: to });
                    }
                }

                // 1 forward moves
                if from.get_delta_rank(to) == -1 {
                    // pushing pawn 1 square
                    if from.get_delta_file(to) == 0 && self.board.is_occupied(to) {
                        return Err(IllegalMoveError::MoveInvalid { mv });
                    }

                    // takes to the right
                    if from.get_delta_file(to) == -1 {
                        // if the previous move was a double pawn move on the file that we are moving
                        // to and it put the pawn next to us.
                        if self.moves.last().is_some_and(|m| {
                            m.get_piece() == Piece::Pawn
                                && m.get_from().get_delta_rank(m.get_to()).abs() == 2
                                && m.get_to().get_file() == to.get_file()
                                && m.get_to().get_rank() == from.get_rank()
                        }) {
                            info!("en-pasent");
                            self.board.remove_piece_from_square(self.moves.last().unwrap().get_to());
                        } else {
                            // takes to the right
                            match self.board.get_piece_on_square(to) {
                                Some((pc, col)) if *col == self.turn => return Err(IllegalMoveError::TakesOwnPiece { mv: mv, piece: *pc }),
                                None => {
                                    return Err(IllegalMoveError::TakesEmptySquare { mv, square: to });
                                }
                                _ => {}
                            }
                        }
                    }
                    // takes to the left
                    if from.get_delta_file(to) == -1 {
                        if self.moves.last().is_some_and(|m| {
                            m.get_piece() == Piece::Pawn
                                && m.get_from().get_delta_rank(m.get_to()).abs() == 2
                                && m.get_to().get_file() == to.get_file()
                                && m.get_to().get_rank() == from.get_rank()
                        }) {
                            info!("en-pasent");
                            self.board.remove_piece_from_square(self.moves.last().unwrap().get_to());
                        } else {
                            match self.board.get_piece_on_square(to) {
                                Some((pc, col)) if *col == self.turn => return Err(IllegalMoveError::TakesOwnPiece { mv: mv, piece: *pc }),
                                None => {
                                    return Err(IllegalMoveError::TakesEmptySquare { mv, square: to });
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
            Piece::King { can_castle } if *p == Piece::King { can_castle: true } => todo!("compute on the fly"),

            Piece::Knight if *p == Piece::Knight => {
                if ATTACK_TABLES.get_attack_pattern_knight(from).contains(to) {
                    if let Some((taken_piece, col)) = self.board.get_piece_on_square(to) {
                        if *col == self.turn {
                            return Err(IllegalMoveError::TakesOwnPiece { mv: mv, piece: *taken_piece });
                        }
                    }
                } else {
                    return Err(IllegalMoveError::MoveInvalid { mv: mv });
                }
            }

            Piece::Bishop if *p == Piece::Bishop => {
                if ATTACK_TABLES.get_attack_pattern_bishop(from, self.board.occupancy).contains(to) {
                    if let Some((taken_piece, col)) = self.board.get_piece_on_square(to) {
                        if *col == self.turn {
                            return Err(IllegalMoveError::TakesOwnPiece { mv: mv, piece: *taken_piece });
                        }
                    }
                } else {
                    return Err(IllegalMoveError::MoveInvalid { mv: mv });
                }
            }

            Piece::Rook if *p == Piece::Rook => {
                if ATTACK_TABLES.get_attack_pattern_rook(from, self.board.occupancy).contains(to) {
                    if let Some((taken_piece, col)) = self.board.get_piece_on_square(to) {
                        if *col == self.turn {
                            return Err(IllegalMoveError::TakesOwnPiece { mv: mv, piece: *taken_piece });
                        }
                    }
                } else {
                    return Err(IllegalMoveError::MoveInvalid { mv: mv });
                }
            }

            Piece::Queen if *p == Piece::Queen => {
                if ATTACK_TABLES.get_attack_pattern_queen(from, self.board.occupancy).contains(to) {
                    if let Some((taken_piece, col)) = self.board.get_piece_on_square(to) {
                        if *col == self.turn {
                            return Err(IllegalMoveError::TakesOwnPiece { mv: mv, piece: *taken_piece });
                        }
                    }
                } else {
                    return Err(IllegalMoveError::MoveInvalid { mv: mv });
                }
            }

            _ => {
                return Err(IllegalMoveError::DifferentPiece {
                    expected: mv.get_piece(),
                    found: *p,
                });
            }
        }
        let (p, c) = self.board.remove_piece_from_square(from).expect("checked that from is Some");

        let takes = self.board.place_piece_on_square(p, c, from).and_then(|(taken, _)| Some(taken));

        mv.set_takes(takes);
        self.moves.push(mv);
        Ok(())
    }
}
