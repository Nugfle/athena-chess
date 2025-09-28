use log::info;
use std::sync::LazyLock;

use attack_tables::AttackTables;
use board::BitBoard;
pub use board::piece::{Color, Piece};
pub use board::square::*;
pub use chess_move::Move;
use error::IllegalMoveError;
pub use mask::BoardMask;

mod attack_tables;
mod board;
mod chess_move;
mod error;
mod evaluation;
mod mask;

static ATTACK_TABLES: LazyLock<AttackTables> = LazyLock::new(|| {
    let start = std::time::Instant::now();
    let at = AttackTables::create_tables();
    let took = start.elapsed().as_millis();
    info!("built attack tables, took {took} ms...");
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

    fn pawn_move(&mut self, mv: &mut Move) -> Result<(), IllegalMoveError> {
        let from = mv.get_from();
        let to = mv.get_to();
        let heading = if self.turn.is_white() { 1 } else { -1 };

        // pawn can move at least 1, at most 2 ranks and only in their forward direction
        if from.get_delta_rank(to).abs() > 2
            || (from.get_delta_rank(to) <= 0 && self.turn.is_white())
            || (from.get_delta_rank(to) >= 0 && self.turn.is_black())
        {
            return Err(IllegalMoveError::MoveInvalid { mv: *mv });
        }
        // handle double moves
        if from.get_delta_rank(to) == 2 * heading {
            // double moves may only happen in a straight line
            if from.get_delta_file(to) != 0 {
                return Err(IllegalMoveError::MoveInvalid { mv: *mv });
            }
            if self.board.is_occupied(from.move_on_file(heading).unwrap()) {
                return Err(IllegalMoveError::Blocked {
                    mv: *mv,
                    square: from.move_on_file(heading).unwrap(),
                });
            }
            if self.board.is_occupied(to) {
                return Err(IllegalMoveError::Blocked { mv: *mv, square: to });
            }
            return Ok(());
        }

        // 1 forward moves
        if from.get_delta_rank(to) == heading {
            // pushing pawn 1 square
            if from.get_delta_file(to) == 0 && self.board.is_occupied(to) {
                return Err(IllegalMoveError::MoveInvalid { mv: *mv });
            }

            // takes to the right
            if from.get_delta_file(to) == 1 {
                // if the previous move was a double pawn move on the file that we are moving
                // to and it put the pawn next to us.
                if self.moves.last().is_some_and(|m| {
                    // the previous move was a double move on the file which we want to take on
                    m.get_piece() == Piece::Pawn
                        && m.get_from().get_delta_rank(m.get_to()).abs() == 2
                        && m.get_from().get_file() == to.get_file()
                        && m.get_to().get_rank() == from.get_rank()
                }) {
                    info!("en-pasent");
                    self.board.remove_piece_from_square(self.moves.last().unwrap().get_to());
                    mv.set_takes(Some(Piece::Pawn));
                } else if self.board.get_piece_on_square(to).is_none() {
                    return Err(IllegalMoveError::TakesEmptySquare { mv: *mv, square: to });
                }
            }
            // takes to the left
            if from.get_delta_file(to) == -1 {
                if self.moves.last().is_some_and(|m| {
                    m.get_piece() == Piece::Pawn
                        && m.get_from().get_delta_rank(m.get_to()).abs() == 2
                        && m.get_from().get_file() == to.get_file()
                        && m.get_to().get_rank() == from.get_rank()
                }) {
                    info!("en-pasent");
                    self.board.remove_piece_from_square(self.moves.last().unwrap().get_to());
                    mv.set_takes(Some(Piece::Pawn));
                } else if self.board.get_piece_on_square(to).is_none() {
                    return Err(IllegalMoveError::TakesEmptySquare { mv: *mv, square: to });
                }
            }
        }
        Ok(())
    }
    fn short_castle(&mut self, from: Square, mv: Move) -> Result<(), IllegalMoveError> {
        let rook_sq = Square::from_rank_file(from.get_rank(), File::H);
        if let Some((pc, col)) = self.board.get_piece_on_square_mut(rook_sq) {
            if *col != self.turn {
                return Err(IllegalMoveError::NotYourPiece {
                    color: *col,
                    square: rook_sq,
                });
            }
            match pc {
                Piece::Rook { has_moved } if !*has_moved => {
                    let f = Square::from_rank_file(from.get_rank(), File::F);
                    let g = Square::from_rank_file(from.get_rank(), File::G);
                    let e = Square::from_rank_file(from.get_rank(), File::E);

                    if self.board.square_is_controlled_by(e, !self.turn) {
                        return Err(IllegalMoveError::IsInCheck);
                    }

                    if self.board.is_occupied(f) || self.board.is_occupied(g) {
                        return Err(IllegalMoveError::Blocked { mv, square: mv.get_to() });
                    }

                    // we have a clear line to an unmoved rook
                    // now we need to check whether the fields the king is moving through are under
                    // attack by an enemy piece
                    if self.board.square_is_controlled_by(f, !self.turn) || self.board.square_is_controlled_by(g, !self.turn) {
                        return Err(IllegalMoveError::MoveInvalid { mv });
                    }

                    let (mut rook, col) = self.board.remove_piece_from_square(rook_sq).unwrap();
                    rook.make_moved();

                    self.board.place_piece_on_square(rook, col, f);
                    Ok(())
                }
                Piece::Rook { has_moved } if *has_moved => Err(IllegalMoveError::MoveInvalid { mv }),
                _ => Err(IllegalMoveError::DifferentPiece {
                    expected: Piece::Rook { has_moved: false },
                    found: *pc,
                }),
            }
        } else {
            Err(IllegalMoveError::EmptySquare { square: rook_sq })
        }
    }

    fn long_castle(&mut self, from: Square, mv: Move) -> Result<(), IllegalMoveError> {
        let rook_sq = Square::from_rank_file(from.get_rank(), File::A);
        if let Some((pc, col)) = self.board.get_piece_on_square(rook_sq) {
            if *col != self.turn {
                return Err(IllegalMoveError::NotYourPiece {
                    color: *col,
                    square: rook_sq,
                });
            }
            match pc {
                Piece::Rook { has_moved } if !has_moved => {
                    let b = Square::from_rank_file(from.get_rank(), File::B);
                    let c = Square::from_rank_file(from.get_rank(), File::C);
                    let d = Square::from_rank_file(from.get_rank(), File::D);
                    let e = Square::from_rank_file(from.get_rank(), File::E);

                    if self.board.is_occupied(b) || self.board.is_occupied(c) || self.board.is_occupied(d) {
                        return Err(IllegalMoveError::Blocked { mv, square: mv.get_to() });
                    }

                    if self.board.square_is_controlled_by(e, !self.turn) {
                        return Err(IllegalMoveError::IsInCheck);
                    }

                    // we have a clear line to an unmoved rook
                    // now we need to check whether the fields the king is moving accross are not
                    // under attack
                    if self.board.square_is_controlled_by(c, !self.turn) || self.board.square_is_controlled_by(d, !self.turn) {
                        return Err(IllegalMoveError::MoveInvalid { mv });
                    }

                    let (rook, col) = self.board.remove_piece_from_square(rook_sq).unwrap();
                    self.board.place_piece_on_square(rook, col, d);
                    Ok(())
                }
                Piece::Rook { has_moved } if *has_moved => Err(IllegalMoveError::MoveInvalid { mv }),
                _ => Err(IllegalMoveError::DifferentPiece {
                    expected: Piece::Rook { has_moved: false },
                    found: *pc,
                }),
            }
        } else {
            Err(IllegalMoveError::EmptySquare { square: rook_sq })
        }
    }

    pub fn execute_move(&mut self, mut mv: Move) -> Result<(), IllegalMoveError> {
        let from = mv.get_from();
        let to = mv.get_to();

        let (p, c) = *match self.board.get_piece_on_square(from) {
            Some(v) => v,
            None => return Err(IllegalMoveError::EmptySquare { square: from }),
        };

        if p != mv.get_piece() {
            return Err(IllegalMoveError::DifferentPiece {
                expected: mv.get_piece(),
                found: p,
            });
        }

        // make sure the color of the piece in the move matches the player whos turn it is
        if c != self.turn {
            return Err(IllegalMoveError::NotYourPiece { color: c, square: from });
        }

        // filter out all moves that would take own piece
        if self.board.get_piece_on_square(to).is_some_and(|(_, col)| *col == c) {
            return Err(IllegalMoveError::TakesOwnPiece {
                mv,
                piece: self.board.get_piece_on_square(to).unwrap().0,
            });
        }

        // check whether the move is valid for the type of piece
        match mv.get_piece() {
            Piece::Pawn => {
                self.pawn_move(&mut mv)?;
            }

            Piece::King { has_moved } => {
                if from.get_delta_rank(to).abs() > 1 {
                    return Err(IllegalMoveError::MoveInvalid { mv });
                }
                // we can at most move 2 squares
                if from.get_delta_file(to).abs() > 2 {
                    return Err(IllegalMoveError::MoveInvalid { mv });
                }
                // handles castling
                if from.get_delta_file(to).abs() == 2 {
                    // if we castle we can only move on the same rank and the king must not have
                    // moved
                    if has_moved || from.get_delta_rank(to) != 0 {
                        return Err(IllegalMoveError::MoveInvalid { mv });
                    } else if from.get_delta_file(to) == -3 {
                        // long castle
                        self.long_castle(from, mv)?;
                    } else if from.get_delta_file(to) == 2 {
                        // short castle
                        self.short_castle(from, mv)?;
                    } else {
                        return Err(IllegalMoveError::MoveInvalid { mv });
                    }
                }
            }

            Piece::Knight => {
                if !ATTACK_TABLES.get_attack_pattern_knight(from).contains(to) {
                    return Err(IllegalMoveError::MoveInvalid { mv });
                }
            }

            Piece::Bishop => {
                if !ATTACK_TABLES.get_attack_pattern_bishop(from, self.board.occupancy).contains(to) {
                    return Err(IllegalMoveError::MoveInvalid { mv });
                }
            }

            Piece::Rook { .. } => {
                if !ATTACK_TABLES.get_attack_pattern_rook(from, self.board.occupancy).contains(to) {
                    return Err(IllegalMoveError::MoveInvalid { mv });
                }
            }

            Piece::Queen => {
                if !ATTACK_TABLES.get_attack_pattern_queen(from, self.board.occupancy).contains(to) {
                    return Err(IllegalMoveError::MoveInvalid { mv });
                }
            }
        }

        let (mut temp_p, temp_c) = self.board.remove_piece_from_square(from).expect("checked that from is Some");
        temp_p.make_moved();

        let takes = self.board.place_piece_on_square(temp_p, temp_c, to).map(|(taken, _)| taken);

        mv.set_takes(takes);
        self.moves.push(mv);
        self.turn = !self.turn;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pawn_double_move() {
        let mut game = Game::init();
        let mv = Move::new(Piece::Pawn, E2, E4, None);
        assert!(game.execute_move(mv).is_ok());
        assert!(game.board.get_piece_on_square(E4).is_some());
        assert!(game.board.get_piece_on_square(E2).is_none());
    }

    #[test]
    fn test_invalid_pawn_move() {
        let mut game = Game::init();
        let mv = Move::new(Piece::Pawn, E2, E5, None);
        assert!(game.execute_move(mv).is_err());
    }

    #[test]
    fn test_knight_move() {
        let mut game = Game::init();
        let mv = Move::new(Piece::Knight, G1, F3, None);
        assert!(game.execute_move(mv).is_ok());
        assert!(game.board.get_piece_on_square(F3).is_some());
        assert!(game.board.get_piece_on_square(G1).is_none());
    }

    #[test]
    fn test_blocked_pawn_move() {
        let mut game = Game::init();
        game.board.place_piece_on_square(Piece::Pawn, Color::Black, E3);
        let mv = Move::new(Piece::Pawn, E2, E4, None);
        assert!(game.execute_move(mv).is_err());
    }

    #[test]
    fn test_pawn_capture() {
        let mut game = Game::init();
        let pawn_push = Move::new(Piece::Pawn, E2, E4, None);
        game.execute_move(pawn_push).unwrap();
        let black_move = Move::new(Piece::Pawn, D7, D5, None);
        game.execute_move(black_move).unwrap();
        let capture = Move::new(Piece::Pawn, E4, D5, None);
        assert!(game.execute_move(capture).is_ok());
        assert_eq!(game.board.get_piece_on_square(D5).unwrap(), &(Piece::Pawn, Color::White));
    }

    #[test]
    fn test_en_passant_left() {
        let mut game = Game::init();
        // White moves e4
        game.execute_move(Move::new(Piece::Pawn, E2, E4, None)).unwrap();
        // Black moves a6 (dummy move)
        game.execute_move(Move::new(Piece::Pawn, A7, A6, None)).unwrap();
        // White moves e5
        game.execute_move(Move::new(Piece::Pawn, E4, E5, None)).unwrap();
        // Black moves d5
        game.execute_move(Move::new(Piece::Pawn, D7, D5, None)).unwrap();
        // White captures en passant
        let en_passant_move = Move::new(Piece::Pawn, E5, D6, None);
        assert!(game.execute_move(en_passant_move).is_ok()); // This should be a valid move in a real game
    }
    #[test]
    fn test_en_passant_right() {
        let mut game = Game::init();
        // White moves e4
        game.execute_move(Move::new(Piece::Pawn, E2, E4, None)).unwrap();
        // Black moves a6 (dummy move)
        game.execute_move(Move::new(Piece::Pawn, A7, A6, None)).unwrap();
        // White moves e5
        game.execute_move(Move::new(Piece::Pawn, E4, E5, None)).unwrap();
        // Black moves d5
        game.execute_move(Move::new(Piece::Pawn, F7, F5, None)).unwrap();
        // White captures en passant
        let en_passant_move = Move::new(Piece::Pawn, E5, F6, None);
        assert!(game.execute_move(en_passant_move).is_ok()); // This should be a valid move in a real game
    }

    #[test]
    fn test_short_castle_white() {
        let mut game = Game::init();
        game.board.remove_piece_from_square(F1);
        game.board.remove_piece_from_square(G1);
        let mv = Move::new(Piece::King { has_moved: false }, E1, G1, None);
        assert!(game.execute_move(mv).is_ok());
        assert_eq!(game.board.get_piece_on_square(G1).unwrap().0, Piece::King { has_moved: true });
        assert_eq!(game.board.get_piece_on_square(F1).unwrap().0, Piece::Rook { has_moved: true });
    }

    #[test]
    fn test_long_castle_white() {
        let mut game = Game::init();
        game.board.remove_piece_from_square(B1);
        game.board.remove_piece_from_square(C1);
        game.board.remove_piece_from_square(D1);
        let mv = Move::new(Piece::King { has_moved: false }, E1, C1, None);
        assert!(game.execute_move(mv).is_ok());
        assert_eq!(game.board.get_piece_on_square(C1).unwrap().0, Piece::King { has_moved: true });
        assert_eq!(game.board.get_piece_on_square(D1).unwrap().0, Piece::Rook { has_moved: true });
    }

    #[test]
    fn test_castle_while_in_check() {
        let mut game = Game::init();
        game.board.remove_piece_from_square(F1);
        game.board.remove_piece_from_square(G1);
        game.board.remove_piece_from_square(E2);
        game.board
            .place_piece_on_square(Piece::Rook { has_moved: false }, Color::Black, E8);
        let mv = Move::new(Piece::King { has_moved: false }, E1, G1, None);
        assert_eq!(game.execute_move(mv), Err(IllegalMoveError::IsInCheck));
    }
}
